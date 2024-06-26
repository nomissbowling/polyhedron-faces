//! Cylinder on polyhedron faces for Rust
//!

use num::Float;

use crate::calc_cg_with_volume;
use crate::Polyhedron;

/// Cylinder
#[derive(Debug)]
pub struct Cylinder<F: Float> {
  /// polyhedron tri: Vec n of Vec 1 indexed triangles
  pub ph: Polyhedron<F>,
  /// edges (duplex)
  pub edges: Vec<(u16, [u16; 5])>
}

/// Cylinder
impl<F: Float + std::fmt::Debug> Cylinder<F> where F: std::iter::Sum {
  /// construct
  pub fn new(r: F, l: F, q: u16) -> Self {
    let o = <F>::from(0).unwrap();
    let h = l / <F>::from(2).unwrap();
    let c = q * 4;
    let c2 = 2 * c;
    let mut vtx = (0..c).into_iter().flat_map(|cn| { // len = c2
      let th = 2.0 * std::f64::consts::PI * cn as f64 / c as f64;
      let (cx, _cy, cz) = (
        r * <F>::from(th.sin()).unwrap(),
        o,
        r * <F>::from(th.cos()).unwrap());
      vec![[cx, -r * h, cz], [cx, r * h, cz]]
    }).collect::<Vec<_>>();
    vtx.push([o, -r * h, o]); // [c2] bottom
    vtx.push([o, r * h, o]); // [c2 + 1] top
    let tri = (0..c).into_iter().flat_map(|cn| {
      let kb = 2 * cn; // not over
      let kt = kb + 1; // not over
      let ktc = (kt + 2) % c2; // not over
      let kbc = (kb + 2) % c2; // not over
      vec![
        vec![[c2, kbc, kb]], // bottom
        vec![[kb, kbc, ktc], [kb, ktc, kt]], // side
        vec![[c2 + 1, kt, ktc]]] // top
    }).collect::<Vec<_>>();
    let (_cg, vol) = calc_cg_with_volume(&tri, &vtx, <F>::from(1e-6).unwrap());
    let edges = vec![];
    Cylinder{ph: Polyhedron{vtx, tri, uv: vec![], vol, center: false}, edges}
  }
}
