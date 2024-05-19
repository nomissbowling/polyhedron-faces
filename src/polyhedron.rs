//! Polyhedron PHF TUV on polyhedron faces for Rust
//!

pub mod tetra;
pub mod cube; // drawstuff(box) dxlib(cube)
pub mod octa;
pub mod sphere; // drawstuff dxlib
pub mod cylinder; // drawstuff
pub mod capsule; // drawstuff dxlib
pub mod cone; // dxlib
pub mod torus;
pub mod pipe;
pub mod pin;
pub mod revolution;

use num::Float;

/// FTVI
#[derive(Debug, Clone)]
pub struct FTVI<F: Float> {
  /// face index of enumerate tri
  pub fi: usize,
  /// triangleindex of enumerate f
  pub ti: usize,
  /// vertex index of enumerate t
  pub vi: usize,
  /// p
  pub p: [F; 3],
  /// uv
  pub uv: [F; 2],
  /// index of vtx
  pub idx: usize
}

/// FTVI
impl<F: Float> FTVI<F> {
  /// puv
  pub fn puv(&self) -> (&[F; 3], &[F; 2]) {
    (&self.p, &self.uv)
  }
}

/// PHF polyhedron face
pub type PHF<F> = Vec<Vec<Vec<FTVI<F>>>>;

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
  fn get_uv_t(&self, fi: usize, ti: usize, vi: usize,
    r: f64, s: f64, o: [f64; 2]) -> [F; 2]; // rot scale offset
  /// ref vtx
  fn ref_vtx(&self) -> &Vec<[F; 3]>;
  /// ref tri
  fn ref_tri(&self) -> &Vec<Vec<[u16; 3]>>;
  /// ref uv
  fn ref_uv(&self) -> &Vec<Vec<[[F; 2]; 3]>>;
  /// centered
  fn centered(&self) -> bool;
  /// with_uv
  fn with_uv(&self, tf: bool) -> PHF<F> { self.phf(tf, self.centered()) }
  /// polyhedron faces by Vec N of Vec P(polygon) indexed triangles
  fn phf(&self, tf: bool, c: bool) -> PHF<F> {
    if tf && self.ref_uv().len() == 0 { // will be duplex checked in get_uv_t
      println!("-- TODO: gen_uv with true expected uv but it is enpty --");
    }
    self.ref_tri().iter().enumerate().map(|(fi, f)|
      f.iter().enumerate().map(|(ti, t)|
        t.iter().enumerate().map(|(vi, &i)| {
          self.gen_uv(i as usize, tf, fi, f.len(), ti, vi, c)
        }).collect()
      ).collect()
    ).collect()
  }
  /// gen uv
  fn gen_uv(&self, i: usize, tf: bool,
    fi: usize, n: usize, ti: usize, vi: usize, c: bool) -> FTVI<F> {
    let r = std::f64::consts::PI / 2.0; // rot
    let s = 1.0f64; // scale
    let o = [0.0f64, 0.0f64]; // offset
    let p = self.ref_vtx()[i];
    let uv = match tf {
    true => self.get_uv_t(fi, ti, vi, 0.0f64, s, o), // on the one texture
    false => self.get_uv_f(n, ti, vi, c, r, s, o) // texture each face
    };
    FTVI::<F>{fi, ti, vi, p, uv, idx: i}
  }
}

/// Polyhedron
#[derive(Debug)]
pub struct Polyhedron<F: Float> {
  /// vtx
  pub vtx: Vec<[F; 3]>,
  /// tri: [n][m] Vec n faces of Vec m indexed triangles
  pub tri: Vec<Vec<[u16; 3]>>,
  /// uv: [n][m] Vec n faces of Vec m uv triangles
  pub uv: Vec<Vec<[[F; 2]; 3]>>,
  /// volume
  pub vol: F,
  /// center
  pub center: bool
}

/// impl trait TUV for Polyhedron
impl<F: Float> TUV<F> for Polyhedron<F> {
  /// get uv from the one texture (fi ti vi: id of expanded polyhedron)
  fn get_uv_t(&self, fi: usize, ti: usize, vi: usize,
    _r: f64, _s: f64, o: [f64; 2]) -> [F; 2] { // TODO: rot scale offset
    if self.uv.len() == 0 {
      return [<F>::from(o[0]).unwrap(), <F>::from(o[1]).unwrap()];
    }
    let uv = self.uv[fi][ti][vi];
    [<F>::from(o[0]).unwrap() + uv[0], <F>::from(o[1]).unwrap() + uv[1]]
  }
  /// ref vtx
  fn ref_vtx(&self) -> &Vec<[F; 3]> { &self.vtx }
  /// ref tri
  fn ref_tri(&self) -> &Vec<Vec<[u16; 3]>> { &self.tri }
  /// ref uv
  fn ref_uv(&self) -> &Vec<Vec<[[F; 2]; 3]>> { &self.uv }
  /// centered
  fn centered(&self) -> bool { self.center }
}
