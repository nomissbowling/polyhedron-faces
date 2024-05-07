//! Sphere on polyhedron faces for Rust
//!

use num::Float;

use crate::Polyhedron;

/// RSphere
#[derive(Debug)]
pub struct RSphere<F: Float> {
  /// polyhedron tri: Vec n of Vec 1 indexed triangles
  pub ph: Polyhedron<F>,
  /// edges (duplex)
  pub edges: Vec<(u16, [u16; 8])>
}

/// RSphere
impl<F: Float + std::fmt::Debug> RSphere<F> {
  /// construct
  pub fn new(r: F, q: u16) -> Self {
    let s = q * 2 + 1;
    let c = q * 4;
    let vtx = (0..s).into_iter().flat_map(|sn| {
      let sth = std::f64::consts::PI * sn as f64 / (s - 1) as f64; // 0 to =pi
      (0..c).into_iter().map(|cn| {
        let cth = 2.0 * std::f64::consts::PI * cn as f64 / c as f64;
        let w = r * <F>::from(sth.sin()).unwrap();
        [
          w * <F>::from(cth.sin()).unwrap(),
          r * <F>::from(-sth.cos()).unwrap(),
          w * <F>::from(cth.cos()).unwrap()]
      }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let tri = (0..s-1).into_iter().flat_map(|sn| { // always below
      (0..c).into_iter().map(|cn| {
        let k = sn * c + cn; // always below
        let ks = k + c; // not over
        let mut ksc = k + c + 1;
        if ksc >= (sn + 2) * c { ksc -= c; } // not over
        let mut kc = k + 1;
        if kc >= (sn + 1) * c { kc -= c; } // not over
        // println!("[{} {} {} {}]", k, kc, ksc, ks);
        vec![[k, kc, ksc], [k, ksc, ks]]
      }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let edges = vec![];
    RSphere{ph: Polyhedron{vtx, tri, uv: vec![], center: false}, edges}
  }
}
