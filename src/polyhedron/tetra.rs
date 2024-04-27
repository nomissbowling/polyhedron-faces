//! Tetra on polyhedron faces for Rust
//!

use num::Float;

use crate::Polyhedron;

/// Tetra
#[derive(Debug)]
pub struct Tetra<F: Float> {
  /// polyhedron tri: Vec 4 of Vec 1 indexed triangles
  pub ph: Polyhedron<F>,
  /// edges (duplex)
  pub edges: Vec<(u8, [u8; 3])>
}

/// Tetra
impl<F: Float + std::fmt::Debug> Tetra<F> {
  /// construct
  pub fn new(r: F) -> Self {
    let vtx = vec![
      [0.5577, -0.1494, -0.2041], // (r6+3r2)/12, (r6-3r2)/12, -r6/12
      [-0.1494, 0.5577, -0.2041], // (r6-3r2)/12, (r6+3r2)/12, -r6/12
      [0.0, 0.0, 0.6124], // 0, 0, r6/4
      [-0.4082, -0.4082, -0.2041] // -r6/6, -r6/6, -r6/12
    ].into_iter().map(|v|
      v.into_iter().map(|f|
        r * <F>::from(f).unwrap()
      ).collect::<Vec<_>>().try_into().unwrap()
    ).collect();
    let tri = vec![
      vec![[3, 1, 0]],
      vec![[3, 2, 1]],
      vec![[3, 0, 2]],
      vec![[2, 0, 1]]
    ];
    let edges = vec![];
    Tetra{ph: Polyhedron{vtx, tri, uv: vec![], center: false}, edges}
  }
}
