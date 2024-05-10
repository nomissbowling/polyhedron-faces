//! Pipe (Tube and HalfPipe) on polyhedron faces for Rust
//!

use num::Float;

use crate::{prec_eq, f_to_f32};
use crate::{Polyhedron, revolution::Revolution, calc_cg, calc_cg_f3};
// use crate::{center_indexed, divide_int};

/// Tube
#[derive(Debug)]
pub struct Tube<F: Float> {
  /// polyhedron tri: Vec n of Vec 1 indexed triangles
  pub ph: Polyhedron<F>,
  /// edges (duplex)
  pub edges: Vec<(u16, [u16; 8])>
}

/// Tube
impl<F: Float + std::fmt::Debug> Tube<F> {
  /// construct
  /// - odm: outer diameter
  /// - idm: inner diameter
  /// - l: length
  /// - q: quality
  pub fn new(odm: F, idm: F, l: F, q: u16) -> Self {
    let r = <F>::from(1).unwrap();
    let z = <F>::from(2).unwrap();
    let (ro, ri) = (odm / z, idm / z);
    let h = l / z;
    let p = 2;
    let mut tbl = vec![(h, ri), (-h, ri), (-h, ro), (h, ro)];
    tbl.push(tbl[0].clone()); // length = p * 2 + 1
    let revo = Revolution::<F>::from_tbl(r, p, q, (false, false), &tbl);
    Tube{ph: revo.ph, edges: revo.edges}
  }
}

/// HalfPipe
#[derive(Debug)]
pub struct HalfPipe<F: Float> {
  /// polyhedron tri: Vec n of Vec 1 indexed triangles
  pub ph: Polyhedron<F>,
  /// edges (duplex)
  pub edges: Vec<(u16, [u16; 8])>
}

/// HalfPipe
impl<F: Float + std::fmt::Debug> HalfPipe<F> where F: std::iter::Sum {
  /// construct
  /// - a: arc angle
  /// - odm: outer diameter
  /// - idm: inner diameter
  /// - l: length
  /// - q: quality
  pub fn new(a: F, odm: F, idm: F, l: F, q: u16) -> Self {
    let o = <F>::from(0).unwrap();
    let z = <F>::from(2).unwrap();
    let (ro, ri) = (odm / z, idm / z);
    let h = l / z;
    let c = q * 4 + 1;
    let fa = a.to_f64().unwrap();
    let vtx = (0..c).into_iter().flat_map(|cn| { // len = c4
      let th = fa * cn as f64 / (c - 1) as f64 - fa / 2.0;
      let (cx, _cy, cz) = (
        <F>::from(th.sin()).unwrap(),
        o,
        <F>::from(th.cos()).unwrap());
      let (xo, zo, xi, zi) = (ro * cx, ro * cz, ri * cx, ri * cz);
      vec![[xi, -h, zi], [xo, -h, zo], [xo, h, zo], [xi, h, zi]]
    }).collect::<Vec<_>>();
    let cg = calc_cg_f3(&vtx, <F>::from(1e-6).unwrap());
    // println!("cg: {:?}", cg); // 0.05825041967286819
    // assert_eq!(f_to_f32(&cg[..2]), &[0.0, 0.0]); // without z
    assert!(prec_eq(&f_to_f32(&cg[..2]), 1e-6, &vec![0.0, 0.0])); // without z
    let vtx = vtx.into_iter().map(|[x, y, z]|
      [x - cg[0], y - cg[1], z - cg[2]]
    ).collect::<Vec<_>>();
    let mut tri = (0..c-1).into_iter().flat_map(|cn| {
      let kn = 4 * (cn + 1);
      (0..4).into_iter().map(|k| { // bottom, outside, top, inside
        let kb = 4 * cn + k; // always below
        let kbc = kb + 4; // not over
        let mut kt = kb + 1;
        if kt >= kn { kt -= 4; } // not over
        let ktc = kt + 4; // not over
        vec![[kb, kbc, ktc], [kb, ktc, kt]]
      }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    tri.push(vec![[0, 1, 2], [0, 2, 3]]); // -a/2 side
    let k = 4 * (c - 1);
    tri.push(vec![[k + 3, k + 2, k + 1], [k + 3, k + 1, k]]); // a/2 side
    let cg = calc_cg(&tri, &vtx, <F>::from(1e-6).unwrap());
    // println!("cg: {:?}", cg); // TODO: 0.009333208976680942 accuracy 0.0 ?
    assert_eq!(f_to_f32(&cg[..2]), &[0.0, 0.0]); // without z
    // assert_eq!(f_to_f32(&cg), &[0.0, 0.0, 0.0]); // expect
    let edges = vec![];
    HalfPipe{ph: Polyhedron{vtx, tri, uv: vec![], center: false}, edges}
  }
}
