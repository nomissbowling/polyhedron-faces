//! Torus on polyhedron faces for Rust
//!

use num::Float;
use qm::q::{TQuaternion, Quaternion};
use qm::v::{TVector, v4::Vector4, v3::Vector3};

use crate::{Polyhedron, revolution::Revolution};

/// Torus
#[derive(Debug)]
pub struct Torus<F: Float> {
  /// polyhedron tri: Vec n of Vec 1 indexed triangles
  pub ph: Polyhedron<F>,
  /// edges (duplex)
  pub edges: Vec<(u16, [u16; 8])>
}

/// Torus
impl<F: Float + std::fmt::Debug + std::iter::Sum> Torus<F> {
  /// construct
  pub fn new(c: F, r: F, p: u16, q: u16) -> Self {
    let p = p * 4;
    let q = q * 4;
    let vtx = (0..p).into_iter().flat_map(|pn| {
      let o = <F>::from(0).unwrap();
      let l = <F>::from(1).unwrap();
      let pi_r = std::f64::consts::PI / 2.0;
      let pth = 2.0 * std::f64::consts::PI * pn as f64 / p as f64;
      let (cx, cy, cz) = (
        c * <F>::from(pth.sin()).unwrap(),
        o,
        c * <F>::from(pth.cos()).unwrap());
      (0..q).into_iter().map(|qn| {
        let qth = 2.0 * std::f64::consts::PI * qn as f64 / q as f64;
        let v = Vector4::<F>::new(&vec![
          r * <F>::from(qth.cos()).unwrap(),
          r * <F>::from(qth.sin()).unwrap(),
          o,
          l]);
        let rot = Quaternion::<F>::from_axis_and_angle(
          &Vector3::<F>::new(&vec![o, l, o]), <F>::from(pth + pi_r).unwrap());
        let d = v.dot_mv(&rot.to_m4_rot());
        [cx + d[0], cy + d[1], cz + d[2]]
      }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let e = p * q;
    let tri = (0..p).into_iter().flat_map(|pn| {
      (0..q).into_iter().map(|qn| {
        let k = pn * q + qn;
        let kp = (k + q) % e;
        let kpq = (k + q + 1) % e;
        let kq = (k + 1) % e;
        vec![[k, kq, kpq], [k, kpq, kp]]
      }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let edges = vec![];
    Torus{ph: Polyhedron{vtx, tri, uv: vec![], center: false}, edges}
  }
}

/// RTorus
#[derive(Debug)]
pub struct RTorus<F: Float> {
  /// polyhedron tri: Vec n of Vec 1 indexed triangles
  pub ph: Polyhedron<F>,
  /// edges (duplex)
  pub edges: Vec<(u16, [u16; 8])>
}

/// RTorus
impl<F: Float + std::fmt::Debug + std::iter::Sum> RTorus<F> {
  /// construct
  pub fn new(c: F, r: F, p: u16, q: u16) -> Self {
    let l = <F>::from(1).unwrap();
    let revo = Revolution::<F>::new(l, p, q, (false, false),
      |n: u16, m: u16| -> (F, F) {
      let k = if n == m - 1 { 0.0 } else { n as f64 / (m - 1) as f64 };
      let th = 2.0 * std::f64::consts::PI * k;
      (r * <F>::from(th.sin()).unwrap(), c + r * <F>::from(th.cos()).unwrap())
    });
    RTorus{ph: revo.ph, edges: revo.edges}
  }
}

/// Ring
#[derive(Debug)]
pub struct Ring<F: Float> {
  /// polyhedron tri: Vec n of Vec 1 indexed triangles
  pub ph: Polyhedron<F>,
  /// edges (duplex)
  pub edges: Vec<(u16, [u16; 8])>
}

/// Ring
impl<F: Float + std::fmt::Debug + std::iter::Sum> Ring<F> {
  /// construct
  pub fn new(c: F, d: F, e: F, p: u16, q: u16) -> Self {
    let l = <F>::from(1).unwrap();
    let revo = Revolution::<F>::new(l, p, q, (false, false),
      |n: u16, m: u16| -> (F, F) {
      let k = if n == m - 1 { 0.0 } else { n as f64 / (m - 1) as f64 };
      let th = 2.0 * std::f64::consts::PI * k;
      (d * <F>::from(th.sin()).unwrap(), c + e * <F>::from(th.cos()).unwrap())
    });
    Ring{ph: revo.ph, edges: revo.edges}
  }
}
