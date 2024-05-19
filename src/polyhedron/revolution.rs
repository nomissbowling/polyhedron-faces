//! Revolution on polyhedron faces for Rust
//!

use num::Float;

use crate::{f_to_f32, adjust_cg_with_volume};
use crate::Polyhedron;
// use crate::{center_indexed, divide_int};

/// Revolution
#[derive(Debug)]
pub struct Revolution<F: Float> {
  /// polyhedron tri: Vec n of Vec 1 indexed triangles
  pub ph: Polyhedron<F>,
  /// edges (duplex)
  pub edges: Vec<(u16, [u16; 8])>
}

/// Revolution
impl<F: Float + std::fmt::Debug> Revolution<F> where F: std::iter::Sum {
  /// construct
  /// - fo: (bottom, top) false: fixed end, true: open end
  pub fn new<Func>(r: F, p: u16, q: u16, fo: (bool, bool), mut f: Func) -> Self
    where Func: FnMut(u16, u16) -> (F, F) {
    let o = <F>::from(0).unwrap();
    let s = p * 2 + 1; // middle = p
    let c = q * 4;
    let cs = c * s;
    let mut vtx = (0..s).into_iter().flat_map(|sn| {
      let g = f(sn, s);
      (0..c).into_iter().map(|cn| {
        let cth = 2.0 * std::f64::consts::PI * cn as f64 / c as f64;
        let w = r * g.1;
        [
          w * <F>::from(cth.sin()).unwrap(),
          r * g.0,
          w * <F>::from(cth.cos()).unwrap()]
      }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let (fo0, mut fo1) = (cs, cs);
    if fo.0 { vtx.push([o, r * f(0, s).0, o]); fo1 += 1; } // [fo0] bottom
    if fo.1 { vtx.push([o, r * f(s - 1, s).0, o]); } // [fo1] top
    let tri = (0..s-1).into_iter().flat_map(|sn| { // always below
      (0..c).into_iter().flat_map(|cn| {
        let k = sn * c + cn; // always below
        let ks = k + c; // not over
        let mut ksc = k + c + 1;
        if ksc >= (sn + 2) * c { ksc -= c; } // not over
        let mut kc = k + 1;
        if kc >= (sn + 1) * c { kc -= c; } // not over
        // println!("[{} {} {} {}]", k, ks, ksc, kc);
        let mut v = vec![];
        if fo.0 && sn == 0 { v.push(vec![[fo0, kc, k]]); } // bottom
        v.push(vec![[k, kc, ksc], [k, ksc, ks]]); // side
        if fo.1 && sn == s - 2 { v.push(vec![[fo1, ks, ksc]]); } // top
        v
      }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let p = <F>::from(1e-6).unwrap();
    let (cg, vol) = adjust_cg_with_volume(&tri, &mut vtx, p);
    assert_eq!(f_to_f32(&[cg[0], cg[2]]), &[0.0, 0.0]); // without y
    let edges = vec![];
    Revolution{ph: Polyhedron{vtx, tri, uv: vec![], vol, center: false}, edges}
  }
  /// construct
  /// - fo: (bottom, top) false: fixed end, true: open end
  pub fn from_tbl(r: F, p: u16, q: u16, fo: (bool, bool), tbl: &Vec<(F, F)>) ->
    Self {
    Revolution::new(r, p, q, fo, |n: u16, m: u16| {
      tbl[n as usize % m as usize]
    })
  }
}
