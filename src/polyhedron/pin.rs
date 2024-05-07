//! Pin on polyhedron faces for Rust
//!

use num::Float;

use crate::{prec_eq, f_to_f32};
use crate::{Polyhedron, revolution::Revolution, calc_cg_f2_x};
// use crate::{center_indexed, divide_int};

/// Pin
#[derive(Debug)]
pub struct Pin<F: Float> {
  /// polyhedron tri: Vec n of Vec 1 indexed triangles
  pub ph: Polyhedron<F>,
  /// edges (duplex)
  pub edges: Vec<(u16, [u16; 8])>
}

/// Pin
impl<F: Float + std::fmt::Debug> Pin<F> {
  /// construct
  pub fn new(r: F, p: u16, q: u16) -> Self {
    let mut tbl = vec![
      [        15.0, 0.800], // 0.000], //
      [14.0+1.0/2.0, 1.870], //
      [        14.0, 2.472], //
      [13.0+1.0/2.0, 2.547],
      [12.0+5.0/8.0, 2.406],
      [11.0+3.0/4.0, 2.094],
      [10.0+7.0/8.0, 1.870],
      [        10.0, 1.797],
      [ 9.0+3.0/8.0, 1.965],
      [ 8.0+5.0/8.0, 2.472],
      [ 7.0+1.0/4.0, 3.703],
      [ 5.0+7.0/8.0, 4.563], // cg ~= 5.0+(64.0+47.0)/128.0 (S 23.284937)
      [ 4.0+1.0/2.0, 4.766],
      [ 3.0+3.0/8.0, 4.510],
      [ 2.0+1.0/4.0, 3.906],
      [     3.0/4.0, 2.828],
      [         0.0, 2.250]];
    tbl.reverse();
    let tbl = tbl.into_iter().map(|[x, y]|
      [<F>::from(x).unwrap(), <F>::from(y).unwrap()]
    ).collect::<Vec<_>>();
    let cg = calc_cg_f2_x(&tbl);
    // println!("cg: {:?}", cg);
    assert!(prec_eq(&f_to_f32(&cg), 1e-6, &vec![5.8672757, 0.0]));
    let tbl = tbl.into_iter().map(|[x, y]|
      (x - cg[0], y - cg[1])
    ).collect::<Vec<_>>();
    assert_eq!(p * 2 + 1, tbl.len() as u16);
    let revo = Revolution::<F>::from_tbl(r, p, q, (true, true), &tbl);
    Pin{ph: revo.ph, edges: revo.edges}
  }
}
