//! Tetra on polyhedron faces for Rust
//!

use num::Float;

use crate::{prec_eq, f_to_f32};
use crate::{Polyhedron, calc_cg, calc_cg_f3};

/// Tetra
#[derive(Debug)]
pub struct Tetra<F: Float> {
  /// polyhedron tri: Vec 4 of Vec 1 indexed triangles
  pub ph: Polyhedron<F>,
  /// edges (duplex)
  pub edges: Vec<(u16, [u16; 3])>
}

/// Tetra
impl<F: Float + std::fmt::Debug> Tetra<F> where F: std::iter::Sum {
  /// construct
  pub fn new(r: F) -> Self {
    let r2 = 2.0f64.sqrt();
    let r6 = 3.0f64.sqrt() * r2; // (not equal to 6.0f64.sqrt() prec 1e-16)
    let o = 0.0f64;
    let a = (r6 + 3.0f64 * r2) / 12.0f64; // 0.5577...
    let b = (r6 - 3.0f64 * r2) / 12.0f64; // -0.1494...
    let c = -r6 / 12.0f64; // -0.2041...
    let d = r6 / 4.0f64; // 0.6124...
    let g = -r6 / 6.0f64; // -0.4082...
    let vtx = vec![
      [a, b, c], // (r6+3r2)/12, (r6-3r2)/12, -r6/12
      [b, a, c], // (r6-3r2)/12, (r6+3r2)/12, -r6/12
      [o, o, d], // 0, 0, r6/4
      [g, g, c] // -r6/6, -r6/6, -r6/12
    ].into_iter().map(|v|
      v.into_iter().map(|f|
        r * <F>::from(f).unwrap()
      ).collect::<Vec<_>>().try_into().unwrap()
    ).collect();
    let cg = calc_cg_f3(&vtx, <F>::from(1e-6).unwrap()); // not accurate
    // println!("cg: {:?}", cg);
    assert!(prec_eq(&f_to_f32(&cg), 1e-6, &vec![0.0, 0.0, 0.0]));
    let tri = vec![
      vec![[3, 1, 0]],
      vec![[3, 2, 1]],
      vec![[3, 0, 2]],
      vec![[2, 0, 1]]
    ];
    let cg = calc_cg(&tri, &vtx, <F>::from(1e-6).unwrap());
    // println!("cg: {:?}", cg);
    assert_eq!(f_to_f32(&cg), &[0.0, 0.0, 0.0]);
    let edges = vec![];
    Tetra{ph: Polyhedron{vtx, tri, uv: vec![], center: false}, edges}
  }
}
