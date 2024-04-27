//! Pipe (Tube and HalfPipe) on polyhedron faces for Rust
//!

use num::Float;

use crate::Polyhedron;
// use crate::{center_indexed, divide_int};

/// Tube
#[derive(Debug)]
pub struct Tube<F: Float> {
  /// polyhedron tri: Vec n of Vec 1 indexed triangles
  pub ph: Polyhedron<F>,
  /// edges (duplex)
  pub edges: Vec<(u8, [u8; 3])>
}

/// Tube TODO: dummy
impl<F: Float + std::fmt::Debug> Tube<F> {
  /// construct
  /// - odm: outer diameter
  /// - idm: inner diameter
  /// - l: length
  /// - q: quality
  pub fn new(odm: F, idm: F, l: F, q: u8) -> Self {
    let r = odm + idm + l + <F>::from(q).unwrap(); // TODO: dummy
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
    Tube{ph: Polyhedron{vtx, tri, uv: vec![], center: false}, edges}
  }
}

/// HalfPipe
#[derive(Debug)]
pub struct HalfPipe<F: Float> {
  /// polyhedron tri: Vec n of Vec 1 indexed triangles
  pub ph: Polyhedron<F>,
  /// edges (duplex)
  pub edges: Vec<(u8, [u8; 3])>
}

/// HalfPipe TODO: dummy
impl<F: Float + std::fmt::Debug> HalfPipe<F> {
  /// construct
  /// - a: arc angle
  /// - odm: outer diameter
  /// - idm: inner diameter
  /// - l: length
  /// - q: quality
  pub fn new(a: F, odm: F, idm: F, l: F, q: u8) -> Self {
    let r = a + odm + idm + l + <F>::from(q).unwrap(); // TODO: dummy
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
    HalfPipe{ph: Polyhedron{vtx, tri, uv: vec![], center: false}, edges}
  }
}
