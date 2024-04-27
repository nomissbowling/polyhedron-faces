#![doc(html_root_url = "https://docs.rs/polyhedron-faces/0.2.1")]
//! polyhedron faces for Rust
//!

pub mod polyhedron;
pub use polyhedron::*;

use num::Float;

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
pub fn center_indexed<F: Float>(idx: &[u8], vtx: &Vec<[F; 3]>) -> [F; 3] {
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
  // use super::polyhedron::tetra::*;
  use super::tetra::*; // short cut

  /// [-- --nocapture] [-- --show-output]
  #[test]
  fn test_tetra() {
    let tetra32_e = Tetra::new(1.0f32);
    let tetra32 = tetra32_e.ph;
    assert_eq!(tetra32.tri.len(), 4);
    assert_eq!(tetra32.vtx.len(), 4);
    let s32 = format!("{:?}", tetra32.vtx[2]);
    println!("{}", s32);
    assert_eq!(s32, "[0.0, 0.0, 0.6124]");

    let tetra64_e = Tetra::new(1.0f64);
    let tetra64 = tetra64_e.ph;
    assert_eq!(tetra64.tri.len(), 4);
    assert_eq!(tetra64.vtx.len(), 4);
    let s64 = format!("{:?}", tetra64.vtx[2]);
    println!("{}", s64);
    assert_eq!(s64, "[0.0, 0.0, 0.6124]");
  }
}
