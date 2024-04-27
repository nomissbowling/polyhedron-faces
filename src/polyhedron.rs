//! Polyhedron PHF TUV on polyhedron faces for Rust
//!

pub mod tetra;
pub mod pipe;
pub mod pin;

use num::Float;

/// PHF polyhedron face
pub type PHF<F> = Vec<Vec<Vec<([F; 3], [F; 2])>>>;

/// trait TUV
pub trait TUV<F: Float> {
  /// get uv from each face (i: vertex id of npolygon)
  fn get_uv_f(&self, n: usize, i: usize, k: usize, c: bool,
    r: f64, s: f64, o: [f64; 2]) -> [F; 2] { // rot scale offset
    if c && k == 0 { // center [0]
      [<F>::from(o[0] + 0.5).unwrap(), <F>::from(o[1] + 0.5).unwrap()]
    } else {
      let (m, j) = match c {
      true => (n, (i + k - 1) % n), // j: c(01) c(12) c(23) c(34) c(40)
      false => (n + 2, if k == 0 { 0 } else { i + k }) // j: 0(12) 0(23) 0(34)
      };
      let t = 2.0 * std::f64::consts::PI * j as f64 / m as f64 + r;
      let uv = [(1.0 + s * t.cos()) / 2.0, 1.0 - (1.0 + s * t.sin()) / 2.0];
      [<F>::from(o[0] + uv[0]).unwrap(), <F>::from(o[1] + uv[1]).unwrap()]
    }
  }
  /// get uv from the one texture (f v i: vertex id of expanded polyhedron)
  fn get_uv_t(&self, f: usize, v: usize, i: usize,
    r: f64, s: f64, o: [f64; 2]) -> [F; 2]; // rot scale offset
  /// ref vtx
  fn ref_vtx(&self) -> &Vec<[F; 3]>;
  /// ref tri
  fn ref_tri(&self) -> &Vec<Vec<[u8; 3]>>;
  /// with_uv
  fn with_uv(&self, tf: bool) -> PHF<F> { self.phf(tf, false) }
  /// polyhedron faces by Vec N of Vec P(polygon) indexed triangles
  fn phf(&self, tf: bool, c: bool) -> PHF<F> {
    self.ref_tri().iter().enumerate().map(|(fi, f)|
      f.iter().enumerate().map(|(vi, v)|
        v.iter().enumerate().map(|(ii, &i)| {
          self.gen_uv(i as usize, tf, fi, f.len(), vi, ii, c)
        }).collect()
      ).collect()
    ).collect()
  }
  /// gen uv
  fn gen_uv(&self, i: usize, tf: bool,
    fi: usize, n: usize, vi: usize, ii: usize, c: bool) -> ([F; 3], [F; 2]) {
    let r = std::f64::consts::PI / 2.0; // rot
    let s = 1.0f64; // scale
    let o = [0.0f64, 0.0f64]; // offset
    let p = self.ref_vtx()[i];
    let uv = match tf {
    true => self.get_uv_t(fi, vi, ii, 0.0f64, s, o), // on the one texture
    false => self.get_uv_f(n, vi, ii, c, r, s, o) // texture each face
    };
    (p, uv)
  }
}

/// Polyhedron
#[derive(Debug)]
pub struct Polyhedron<F: Float> {
  /// vtx
  pub vtx: Vec<[F; 3]>,
  /// tri: [n][m] Vec n faces of Vec m indexed triangles
  pub tri: Vec<Vec<[u8; 3]>>
}

/// impl trait TUV for Polyhedron
impl<F: Float> TUV<F> for Polyhedron<F> {
  /// get uv from the one texture (f v i: vertex id of expanded polyhedron)
  fn get_uv_t(&self, _f: usize, _v: usize, _i: usize,
    _r: f64, _s: f64, o: [f64; 2]) -> [F; 2] { // rot scale offset
    [<F>::from(o[0] + 0.0).unwrap(), <F>::from(o[1] + 0.0).unwrap()]
  }
  /// ref vtx
  fn ref_vtx(&self) -> &Vec<[F; 3]> { &self.vtx }
  /// ref tri
  fn ref_tri(&self) -> &Vec<Vec<[u8; 3]>> { &self.tri }
}
