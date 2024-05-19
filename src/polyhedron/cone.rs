//! Cone on polyhedron faces for Rust
//!

use num::Float;

use crate::calc_cg_with_volume;
use crate::Polyhedron;

/// Cone
#[derive(Debug)]
pub struct Cone<F: Float> {
  /// polyhedron tri: Vec n of Vec 1 indexed triangles
  pub ph: Polyhedron<F>,
  /// edges (duplex)
  pub edges: Vec<(u16, [u16; 4])>
}

/// Cone
impl<F: Float + std::fmt::Debug> Cone<F> where F: std::iter::Sum {
  /// construct
  pub fn new(r: F, h: F, q: u16) -> Self {
    let o = <F>::from(0).unwrap();
    let b = h / <F>::from(-4.0).unwrap();
    let t = h * <F>::from(3.0 / 4.0).unwrap();
    let c = q * 4;
    let mut vtx = (0..c).into_iter().map(|cn| {
      let th = 2.0 * std::f64::consts::PI * cn as f64 / c as f64;
      [r * <F>::from(th.sin()).unwrap(), b, r * <F>::from(th.cos()).unwrap()]
    }).collect::<Vec<_>>();
    vtx.push([o, r * b, o]); // [c] bottom
    vtx.push([o, r * t, o]); // [c + 1] top
    let tri = (0..c).into_iter().flat_map(|cn| {
      let ck = (cn + 1) % c;
      vec![vec![[c, ck, cn]], vec![[c + 1, cn, ck]]] // bottom, top
    }).collect::<Vec<_>>();
    let (_cg, vol) = calc_cg_with_volume(&tri, &vtx, <F>::from(1e-6).unwrap());
    let edges = vec![];
    Cone{ph: Polyhedron{vtx, tri, uv: vec![], vol, center: false}, edges}
  }
}
