//! Octa on polyhedron faces for Rust
//!

use num::Float;

use crate::calc_cg_with_volume;
use crate::{Polyhedron, cube::CubeCenter};

/// Octa
#[derive(Debug)]
pub struct Octa<F: Float> {
  /// polyhedron tri: Vec 8 of Vec 1 indexed triangles
  pub ph: Polyhedron<F>,
  /// edges (duplex)
  pub edges: Vec<(u16, [u16; 4])>
}

/// Octa
impl<F: Float + std::fmt::Debug> Octa<F> where F: std::iter::Sum {
  /// construct
  pub fn new(r: F) -> Self {
    let cubec = CubeCenter::<F>::new(r);
    let vtx = (24..24+6).into_iter().map(|i| cubec.ph.vtx[i]).collect();
    let tri = vec![
      vec![[0, 2, 4]],
      vec![[0, 4, 3]],
      vec![[0, 3, 5]],
      vec![[0, 5, 2]],
      vec![[1, 5, 3]],
      vec![[1, 2, 5]],
      vec![[1, 4, 2]],
      vec![[1, 3, 4]]
    ];
    let (_cg, vol) = calc_cg_with_volume(&tri, &vtx, <F>::from(1e-6).unwrap());
    let edges = vec![];
    Octa{ph: Polyhedron{vtx, tri, uv: vec![], vol, center: false}, edges}
  }
}
