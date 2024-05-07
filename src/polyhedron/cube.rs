//! Cube on polyhedron faces for Rust
//!

use num::Float;

use crate::{Polyhedron, center_indexed, center_indexed_uv};

/// Cube
#[derive(Debug)]
pub struct Cube<F: Float> {
  /// polyhedron tri: Vec 6 of Vec 2 indexed triangles
  pub ph: Polyhedron<F>,
  /// edges (duplex)
  pub edges: Vec<(u16, [u16; 3])>
}

/// Cube
impl<F: Float + std::fmt::Debug> Cube<F> {
  /// construct
  pub fn new(r: F) -> Self {
    let vtx = vec![
      [r, -r, r], [r, -r, -r], [r, r, -r], [r, r, r], // +X (1 0 0) right
      [-r, -r, r], [-r, r, r], [-r, r, -r], [-r, -r, -r], // -X (-1 0 0) left
      [r, r, -r], [-r, r, -r], [-r, r, r], [r, r, r], // +Y (0 1 0) back
      [r, -r, -r], [r, -r, r], [-r, -r, r], [-r, -r, -r], // -Y (0 -1 0) front
      [-r, r, r], [-r, -r, r], [r, -r, r], [r, r, r], // +Z (0 0 1) top
      [-r, r, -r], [r, r, -r], [r, -r, -r], [-r, -r, -r] // -Z (0 0 -1) bottom
    ];
    let tri = (0..6).into_iter().map(|f| {
      let k = f * 4;
      vec![[k, k + 1, k + 2], [k, k + 2, k + 3]]
    }).collect();
    let edges = vec![];
    let uv = vec![
/*
      [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]], // +X (1 0 0) right
      [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]], // -X (-1 0 0) left
      [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]], // +Y (0 1 0) back
      [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]], // -Y (0 -1 0) front
      [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]], // +Z (0 0 1) top
      [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]] // -Z (0 0 -1) bottom
*/
      [[0.25, 0.50], [0.50, 0.50], [0.50, 0.25], [0.25, 0.25]], // 5137
      [[0.25, 0.75], [0.25, 1.00], [0.50, 1.00], [0.50, 0.75]], // 4620
      [[0.50, 0.25], [0.50, 0.00], [0.25, 0.00], [0.25, 0.25]], // 3267
      [[0.50, 0.50], [0.25, 0.50], [0.25, 0.75], [0.50, 0.75]], // 1540
      [[0.00, 0.25], [0.00, 0.50], [0.25, 0.50], [0.25, 0.25]], // 6457
      [[0.75, 0.75], [0.75, 0.50], [0.50, 0.50], [0.50, 0.75]] // 2310
    ].into_iter().map(|f|
      (0..2).into_iter().map(|t| {
        let i = [[0, 1, 2], [0, 2, 3]];
        i[t].into_iter().map(|k|
          f[k].iter().map(|&p|
            <F>::from(p).unwrap()
          ).collect::<Vec<_>>().try_into().unwrap()
        ).collect::<Vec<_>>().try_into().unwrap()
      }).collect()
    ).collect();
    Cube{ph: Polyhedron{vtx, tri, uv, center: false}, edges}
  }
}

/// CubeCenter
#[derive(Debug)]
pub struct CubeCenter<F: Float> {
  /// polyhedron tri: Vec 6 of Vec 4 indexed triangles
  pub ph: Polyhedron<F>,
  /// edges (duplex)
  pub edges: Vec<(u16, [u16; 3])>
}

/// CubeCenter
impl<F: Float + std::fmt::Debug> CubeCenter<F> {
  /// construct
  pub fn new(r: F) -> Self {
    let cube = Cube::<F>::new(r);
    let mut vtx = cube.ph.vtx.clone();
    for f in 0..6 {
      let idxs = (0..4).into_iter().map(|i| f * 4 + i).collect::<Vec<_>>();
      vtx.push(center_indexed(&idxs, &cube.ph.vtx));
    }
    let tri = (0..6).into_iter().map(|f| {
      let o = f + 24;
      let k = f * 4;
      vec![[o, k, k + 1], [o, k + 1, k + 2], [o, k + 2, k + 3], [o, k + 3, k]]
    }).collect();
    let edges = vec![];
    let uv = (0..6).into_iter().map(|f| {
      let uvr = &cube.ph.uv[f]; // [[0 1 2] [0 2 3]]
      let uvs = vec![uvr[0][0], uvr[0][1], uvr[0][2], uvr[1][2]]; // [0 1 2 3]
      let idxs = (0..4).into_iter().collect::<Vec<_>>();
      let o = center_indexed_uv(&idxs, &uvs);
      vec![
        [o, uvs[0], uvs[1]],
        [o, uvs[1], uvs[2]],
        [o, uvs[2], uvs[3]],
        [o, uvs[3], uvs[0]]]
    }).collect();
    CubeCenter{ph: Polyhedron{vtx, tri, uv, center: true}, edges}
  }
}
