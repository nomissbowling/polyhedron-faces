//! Capsule on polyhedron faces for Rust
//!

use num::Float;

use crate::Polyhedron;

/// Capsule
#[derive(Debug)]
pub struct Capsule<F: Float> {
  /// polyhedron tri: Vec n of Vec 1 indexed triangles
  pub ph: Polyhedron<F>,
  /// edges (duplex)
  pub edges: Vec<(u16, [u16; 8])>
}

/// Capsule
impl<F: Float + std::fmt::Debug> Capsule<F> {
  /// construct
  pub fn new(r: F, l: F, q: u16) -> Self {
    let h = l / <F>::from(2).unwrap();
    let s = q * 2 + 1; // middle = q
    let c = q * 4;
    let vtx = (0..s+1).into_iter().flat_map(|sn| { // duplex middle q (s+1)
      let ns = if sn <= q { sn } else { sn - 1 };
      let sth = std::f64::consts::PI * ns as f64 / (s - 1) as f64; // 0 to =pi
      (0..c).into_iter().map(|cn| {
        let cth = 2.0 * std::f64::consts::PI * cn as f64 / c as f64;
        let w = r * <F>::from(sth.sin()).unwrap();
        [
          w * <F>::from(cth.sin()).unwrap(),
          r * (<F>::from(-sth.cos()).unwrap() + if sn <= q { -h } else { h }),
          w * <F>::from(cth.cos()).unwrap()]
      }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let tri = (0..s).into_iter().flat_map(|sn| { // always below (s+1-1)
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
    Capsule{ph: Polyhedron{vtx, tri, uv: vec![], center: false}, edges}
  }
}
