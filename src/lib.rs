#![doc(html_root_url = "https://docs.rs/polyhedron-faces/0.3.2")]
//! polyhedron faces for Rust
//!

pub mod polyhedron;
pub use polyhedron::*;

use num::Float;
use qm::v::TVector;

/// sum of vec [F; 2] without trait Sum
/// when use += need trait Float + std::ops::AddAssign &lt; [F; 2] &gt;
pub fn sum_f2<F: Float>(uvs: &Vec<[F; 2]>) -> Vec<F> {
  uvs.iter().fold(vec![<F>::from(0).unwrap(); 2], |s, p|
    s.iter().zip(p.iter()).map(|(&q, &p)| q + p).collect())
}

/// avg of vec [F; 2]
pub fn avg_f2<F: Float>(uvs: &Vec<[F; 2]>) -> Vec<F> {
  let n = <F>::from(uvs.len()).unwrap();
  sum_f2(uvs).iter().map(|&p| p / n).collect()
}

/// center indexed uv
pub fn center_indexed_uv<F: Float>(idx: &[u16], uvs: &Vec<[F; 2]>) -> [F; 2] {
  let p = avg_f2(&idx.iter().map(|&i| uvs[i as usize]).collect());
  p.as_slice().try_into().unwrap()
}

/// sum of vec [F; 3] without trait Sum
/// when use += need trait Float + std::ops::AddAssign &lt; [F; 3] &gt;
pub fn sum_f3<F: Float>(vs: &Vec<[F; 3]>) -> Vec<F> {
  vs.iter().fold(vec![<F>::from(0).unwrap(); 3], |s, p|
    s.iter().zip(p.iter()).map(|(&q, &p)| q + p).collect())
}

/// avg of vec [F; 3]
pub fn avg_f3<F: Float>(vs: &Vec<[F; 3]>) -> Vec<F> {
  let n = <F>::from(vs.len()).unwrap();
  sum_f3(vs).iter().map(|&v| v / n).collect()
}

/// center indexed
pub fn center_indexed<F: Float>(idx: &[u16], vtx: &Vec<[F; 3]>) -> [F; 3] {
  let p = avg_f3(&idx.iter().map(|&i| vtx[i as usize]).collect());
  p.as_slice().try_into().unwrap()
}

/// divide internally
pub fn divide_int<F: Float>(p: &[F; 3], q: &[F; 3], m: i32, n: i32) -> [F; 3] {
  let mf = <F>::from(m).unwrap();
  let nf = <F>::from(n).unwrap();
  let sf = mf + nf;
  p.iter().zip(q.iter()).map(|(&a, &b)|
    (nf * a + mf * b) / sf).collect::<Vec<_>>().as_slice().try_into().unwrap()
}

/// divide externally
pub fn divide_ext<F: Float>(p: &[F; 3], q: &[F; 3], m: i32, n: i32) -> [F; 3] {
  divide_int(p, q, m, -n)
}

/// solve quadratic equation
pub fn solve<F: Float>(a: F, b: F, c: F) -> [F; 2] {
  let z = <F>::from(2).unwrap();
  let d = b * b - <F>::from(4).unwrap() * a * c;
  let r = d.sqrt();
  [(-b - r) / (z * a), (-b + r) / (z * a)]
}

/// solve quadratic equation (b2 = b / 2)
pub fn solve2<F: Float>(a: F, b2: F, c: F) -> [F; 2] {
  let d = b2 * b2 - a * c;
  let r = d.sqrt();
  [(-b2 - r) / a, (-b2 + r) / a]
}

/// sol
pub fn sol<F: Float>(r: &[F], s: F, e: F) -> F {
  for i in 0..r.len() {
    if r[i] >= s && r[i] <= e { return r[i]; }
  }
  <F>::from(0).unwrap()
}

/// calc cg f2 x axis
pub fn calc_cg_f2_x<F: Float>(vs: &Vec<[F; 2]>) -> Vec<F> {
  let o = <F>::from(0).unwrap();
  let z = <F>::from(2).unwrap();
  let mut stk = (1..vs.len()).into_iter().map(|vn| {
    let (a, b) = (vs[vn - 1], vs[vn]);
    [(b[0] - a[0]) * (b[1] + a[1]) / z, o]
  }).collect::<Vec<_>>();
  stk.push([o, o]);
  for vn in 0..vs.len() {
    stk[vn][1] = if vn == 0 { o } else { stk[vn - 1][0] + stk[vn - 1][1] };
  }
  let middle = stk[stk.len() - 1][1] / z;
  let mut n = 0;
  for vn in 0..vs.len() { if stk[vn][1] >= middle { n = vn; break; } }
  let m = (middle - stk[n - 1][1]) / (stk[n][1] - stk[n - 1][1]);
  let (h0, h1) = (vs[n - 1][0], vs[n][0]);
  let (w0, w1) = (vs[n - 1][1], vs[n][1]);
  let h = h1 - h0;
  // vec![m * h + h0, o] // fast approximation (not accurate)
  let r = solve2(w1 - w0, h * w0, -m * h * h * (w1 + w0));
  vec![sol(&r, o, h) + h0, o] // TODO: y = o
}

/// calc cg f3 CAUTION not accurate
/// (depends on density of vertices because no care of mass volume)
/// - vs: length &ge; 3
/// - p: precision for equality
pub fn calc_cg_f3<F: Float>(vs: &Vec<[F; 3]>, p: F) -> Vec<F> {
  let mut vtmp = Vec::<[F; 3]>::new();
  for i in 0..vs.len() {
    let mut found = false;
    for v in vtmp.iter() { if prec_eq(v, p, &vs[i]) { found = true; break; } }
    if !found { vtmp.push(vs[i]); }
  }
  round_prec(&avg_f3(&vtmp), p, <F>::from(0).unwrap()) // not accurate
}

/// calc cg
/// - idx: index of triangles on each faces
/// - vtx: length &ge; 3
/// - p: precision for equality
/// when use += need trait Float + std::ops::AddAssign &lt; [F; 3] &gt;
pub fn calc_cg<F: Float + std::fmt::Debug>(
  idx: &Vec<Vec<[u16; 3]>>, vtx: &Vec<[F; 3]>, p: F) -> Vec<F>
  where F: std::iter::Sum {
  let o = <F>::from(0).unwrap();
  let mut m_total = o; // 6 * volume
  let mut moi = [o, o, o];
  for f in idx.iter() {
    for t in f.iter() {
      let vs = (0..t.len()).into_iter().map(|i|
        vtx[t[i] as usize]).collect::<Vec<_>>();
      let m = vs[2].dot(&vs[0].cross(&vs[1]));
      m_total = m_total + m; // +=
      let c = calc_cg_o(&vs);
      for j in 0..moi.len() { moi[j] = moi[j] + m * c[j]; } // +=
    }
  }
  let cg = moi.into_iter().map(|p| p / m_total).collect::<Vec<_>>();
  // println!("CG: {:?}", cg);
  round_prec(&cg, p, <F>::from(0).unwrap())
}

/// calc cg skip o
/// - vs: length = 3 (It means: (o + vs[0] + vs[1] + vs[2]) / 4)
pub fn calc_cg_o<F: Float>(vs: &Vec<[F; 3]>) -> Vec<F> {
  let n = <F>::from(4).unwrap(); // always 4
  sum_f3(vs).iter().map(|&v| v / n).collect()
}

/// round precision
pub fn round_prec<F: Float>(v: &[F], e: F, q: F) -> Vec<F> {
  let o = <F>::from(0).unwrap();
  v.iter().map(|&p| {
    if (p - q).abs() >= e { p - q } else { o }
  }).collect()
}

/// check equal with precision
pub fn prec_eq<F: Float>(s: &[F], e: F, d: &[F]) -> bool {
  for i in 0..s.len() {
    if (s[i] - d[i]).abs() >= e { return false; }
  }
  true
}

/// f_to_f32
pub fn f_to_f32<F: Float>(v: &[F]) -> Vec<f32> {
  v.iter().map(|i| i.to_f32().unwrap()).collect()
}

/// f_to_f64
pub fn f_to_f64<F: Float>(v: &[F]) -> Vec<f64> {
  v.iter().map(|i| i.to_f64().unwrap()).collect()
}

/// tests
#[cfg(test)]
mod tests {
  use super::{prec_eq, f_to_f32};
  use super::polyhedron::TUV;
  // use super::polyhedron::tetra::*;
  use super::tetra::*; // short cut
  use super::cube::*;
  use super::octa::*;
  use super::pipe::*;
  use super::polyhedron; // polyhedron::pin::Pin

  /// [-- --nocapture] [-- --show-output]
  #[test]
  fn test_tetra() {
    let tetra32_e = Tetra::new(1.0f32);
    let tetra32 = tetra32_e.ph;
    assert_eq!(tetra32.tri.len(), 4);
    assert_eq!(tetra32.vtx.len(), 4);
    let s32 = format!("{:?}", tetra32.vtx[2]);
    println!("{}", s32);
    assert_eq!(s32, "[0.0, 0.0, 0.61237246]"); // TODO: prec_eq

    let tetra64_e = Tetra::new(1.0f64);
    let tetra64 = tetra64_e.ph;
    assert_eq!(tetra64.tri.len(), 4);
    assert_eq!(tetra64.vtx.len(), 4);
    let s64 = format!("{:?}", tetra64.vtx[2]);
    println!("{}", s64);
    assert_eq!(s64, "[0.0, 0.0, 0.6123724356957946]"); // TODO: prec_eq
  }

  #[test]
  fn test_cube() {
    let cube32_e = Cube::new(1.0f32);
    let cube32 = cube32_e.ph;
    assert_eq!(cube32.tri.len(), 6);
    assert_eq!(cube32.tri[0].len(), 2);
    assert_eq!(cube32.vtx.len(), 6 * 4);
    let s32 = format!("{:?}", cube32.vtx[23]);
    println!("{}", s32);
    assert_eq!(s32, "[-1.0, -1.0, -1.0]");

    let uv32 = format!("{:?}", cube32.with_uv(false)[0][0][0].puv());
    println!("{}", uv32);
    assert_eq!(uv32, "([1.0, -1.0, 1.0], [0.5, 0.0])");

    let uv32 = format!("{:?}", cube32.with_uv(false)[5][1][2].puv());
    println!("{}", uv32);
    assert_eq!(uv32, "([-1.0, -1.0, -1.0], [1.0, 0.5])");

    let uv32 = format!("{:?}", cube32.with_uv(true)[0][0][0].puv());
    println!("{}", uv32);
    assert_eq!(uv32, "([1.0, -1.0, 1.0], [0.25, 0.5])"); // not use 0.50

    let uv32 = format!("{:?}", cube32.with_uv(true)[5][1][2].puv());
    println!("{}", uv32);
    assert_eq!(uv32, "([-1.0, -1.0, -1.0], [0.5, 0.75])"); // not use 0.50
  }

  #[test]
  fn test_cube_center() {
    let cube32_e = CubeCenter::new(1.0f32);
    let cube32 = cube32_e.ph;
    assert_eq!(cube32.tri.len(), 6);
    assert_eq!(cube32.tri[0].len(), 4);
    assert_eq!(cube32.vtx.len(), 6 * 4 + 6);
    let s32 = format!("{:?}", cube32.vtx[23]);
    println!("{}", s32);
    assert_eq!(s32, "[-1.0, -1.0, -1.0]");

    let s32 = format!("{:?}", cube32.vtx[29]);
    println!("{}", s32);
    assert_eq!(s32, "[0.0, 0.0, -1.0]");

    let uv32 = format!("{:?}", cube32.with_uv(false)[0][0][0].puv());
    println!("{}", uv32);
    assert_eq!(uv32, "([1.0, 0.0, 0.0], [0.5, 0.5])");

    let uv32 = format!("{:?}", cube32.with_uv(false)[5][3][2].puv());
    println!("{}", uv32);
    assert_eq!(uv32, "([-1.0, 1.0, -1.0], [0.5, 0.0])");

    let uv32 = format!("{:?}", cube32.with_uv(true)[0][0][0].puv());
    println!("{}", uv32);
    assert_eq!(uv32, "([1.0, 0.0, 0.0], [0.375, 0.375])");

    let uv32 = format!("{:?}", cube32.with_uv(true)[5][3][2].puv());
    println!("{}", uv32);
    assert_eq!(uv32, "([-1.0, 1.0, -1.0], [0.75, 0.75])");
  }

  #[test]
  fn test_octa() {
    let octa32_e = Octa::new(1.0f32);
    let octa32 = octa32_e.ph;
    assert_eq!(octa32.tri.len(), 8);
    assert_eq!(octa32.vtx.len(), 6);
    let s32 = format!("{:?}", octa32.vtx[0]);
    println!("{}", s32);
    assert_eq!(s32, "[1.0, 0.0, 0.0]");
  }

  #[test]
  fn test_tube() {
    let tube32_e = Tube::new(0.5, 0.4, 1.0, 6);
    let tube32 = tube32_e.ph;
    assert_eq!(tube32.tri.len(), 4 * (6*4));
    assert_eq!(tube32.vtx.len(), (2*2+1) * (6*4));
  }

  #[test]
  fn test_halfpipe() {
    let cmp = [0.0, 0.5, 0.2 - 0.05825041967286819]; // [0, l/2, idm/2 - cg]
    let halfpipe32_e = HalfPipe::new(4.712388980, 0.5, 0.4, 1.0, 6); // 3pi/2
/*
    let cmp = [0.0, 0.5, 0.2]; // [0, l/2, idm/2]
    let halfpipe32_e = HalfPipe::new(6.283185307, 0.5, 0.4, 1.0, 6); // 2pi
*/
    let halfpipe32 = halfpipe32_e.ph;
    assert_eq!(halfpipe32.tri.len(), 4 * (6*4) + 2);
    assert_eq!(halfpipe32.vtx.len(), 4 * (6*4+1));
    let middle32 = halfpipe32.vtx[4 * (6*4/2) + 3]; // middle idm top
    println!("{:?}", middle32);
    assert!(prec_eq(&f_to_f32(&middle32), 1e-6, &cmp));
  }

  #[test]
  fn test_pin() {
    let pin32_e = polyhedron::pin::Pin::new(1.0f32, 8, 6);
    let pin32 = pin32_e.ph;
    assert_eq!(pin32.tri.len(), 432);
    assert_eq!(pin32.vtx.len(), 410); // (8*2+1) * (6*4) + 2 (bottom, top)
    let btm32 = pin32.vtx[(8*2+1) * (6*4)]; // bottom
    println!("{:?}", btm32);
    assert!(prec_eq(&f_to_f32(&btm32), 1e-6, &[0.0, -5.8672757, 0.0]));
  }
}
