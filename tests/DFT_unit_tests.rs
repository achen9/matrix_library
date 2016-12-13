//! NAME
//!  DFT_unit_tests.rs
//!
//! DESCRIPTION
//!  Unit test file for DFT class methods
//!
//! PARAMETERS
//!  See specific method
//!
//! RETURN
//!  See specific method
//! 
//! EXAMPLE
//!  See specific method
//!
//! NOTES
//!  Make sure the Cargo.toml file is available to compile the code
//!
//! Alex Z. Chen - 12/12/2016
//! alexac9@uw.edu
//!
extern crate matrix_lib;
// A tolerance constant is needed when comparing doubles. Due to rounding
// errors, doing a straight comparison of doubles may cause erroneous 
// results. Checking the two doubles are within some tolerance is a better
// method to check for equality.
const TOLERANCE: f64 = 0.00001;
const PI: f64 = 3.14159;

// Constructor Test: Test if 2/3 can be assigned to a complex 
#[test]
fn constructor_test() {
  use matrix_lib::complex::{Complex, complex};
  use matrix_lib::DFT::{DFT, dft};
  let d: DFT = dft(10);
  assert!(10 == d.npts());
}
// Transform Matrix Method Test: Spot check transform matrix elements are 1+0j
#[test]
fn transform_matrix_method_test() {
  use matrix_lib::complex::{Complex, complex};
  use matrix_lib::DFT::{DFT, dft};
  let mut d: DFT = dft(4);
  let c = complex(1.0, 0.0);
  assert!(c == d.transform_matrix().get(0, 0));
  assert!(c == d.transform_matrix().get(2, 0));
  assert!(c == d.transform_matrix().get(0, 3));
}
// Unitary Matrix Method Test: Spot check unitary matrix elements are 0.5+0j
#[test]
fn unitary_matrix_method_test() {
  use matrix_lib::complex::{Complex, complex};
  use matrix_lib::DFT::{DFT, dft};
  let mut d: DFT = dft(4);
  let c = complex(0.5, 0.0);
  assert!(c == d.unitary_matrix().get(0, 0));
  assert!(c == d.unitary_matrix().get(2, 0));
  assert!(c == d.unitary_matrix().get(0, 3));
}
// Inverse Matrix Method Test: Spot check inverse matrix elements are 0-0.5j
#[test]
fn inverse_matrix_method_test() {
  use matrix_lib::complex::{Complex, complex};
  use matrix_lib::DFT::{DFT, dft};
  let mut d: DFT = dft(4);
  let c = complex(0.0, -0.5);
  assert!(c == d.inverse_matrix().get(1, 3));
  assert!(c == d.inverse_matrix().get(3, 1));
}
// Discrete Fourier Transform Test: Check DFT can be computed
#[test]
fn dft_tfm_method_test() {
  use matrix_lib::complex::{Complex, complex};
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::DFT::{DFT, dft};
  let mut d: DFT = dft(4);
  let c1 = complex(0.0, -0.5); let c2 = complex(1.2, -3.2);
  let c3 = complex(3.4, 5.6);  let c4 = complex(5.6, 9.4);
  let mut v: Matrix<Complex> = matrix(4, 1);
  v.set(0, 0, c1); v.set(1, 0, c2); v.set(2, 0, c3); v.set(3, 0, c4);
  let o: Matrix<Complex> = d.dft_tfm(v);
  println!("DFT is: {}", o);
}
// Inverse Discrete Fourier Transform Test: Check inverse DFT can be computed
#[test]
fn dft_inv_method_test() {
  use matrix_lib::complex::{Complex, complex};
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::DFT::{DFT, dft};
  let mut d: DFT = dft(4);
  let c1 = complex(0.0, -0.5); let c2 = complex(1.2, -3.2);
  let c3 = complex(3.4, 5.6);  let c4 = complex(5.6, 9.4);
  let mut v: Matrix<Complex> = matrix(4, 1);
  v.set(0, 0, c1); v.set(1, 0, c2); v.set(2, 0, c3); v.set(3, 0, c4);
  let o: Matrix<Complex> = d.dft_inv(v);
  println!("DFT inverse is: {}", o);
}
// DFT and Inverse DFT Test: Check original input can be recovered by applying DFT and inverse DFT
#[test]
fn dft_test() {
  use matrix_lib::complex::{Complex, complex};
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::DFT::{DFT, dft};
  let mut d: DFT = dft(4);
  let c1 = complex(0.0, -0.5); let c2 = complex(1.2, -3.2);
  let c3 = complex(3.4, 5.6);  let c4 = complex(5.6, 9.4);
  let mut v: Matrix<Complex> = matrix(4, 1);
  v.set(0, 0, c1); v.set(1, 0, c2); v.set(2, 0, c3); v.set(3, 0, c4);
  let tfm: Matrix<Complex> = d.dft_tfm(v.clone());
  let v_chk: Matrix<Complex> = d.dft_inv(tfm);
  assert!(v == v_chk);
}
// Unitary Matrix Test: Check unitary matrix raised to the 4th power is approximately the identity matrix
#[test]
fn unitary_matrix_power_test() {
  use matrix_lib::complex::{Complex, complex};
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::DFT::{DFT, dft};
  let mut d: DFT = dft(4);
  let u: Matrix<Complex> = d.unitary_matrix();
  let u4: Matrix<Complex> = u.clone() * u.clone() * u.clone() * u.clone();
  let one = complex(1.0, 0.0);
  let zero = complex(0.0, 0.0);
  assert!(one == u4.get(0, 0));
  assert!(one == u4.get(1, 1));
  assert!(one == u4.get(2, 2));
  assert!(one == u4.get(3, 3));
  assert!(zero == u4.get(0, 1));
  assert!(zero == u4.get(0, 2));
  assert!(zero == u4.get(0, 3));
  assert!(zero == u4.get(1, 0));
  assert!(zero == u4.get(1, 2));
  assert!(zero == u4.get(1, 3));
  assert!(zero == u4.get(2, 0));
  assert!(zero == u4.get(2, 1));
  assert!(zero == u4.get(2, 3));
  assert!(zero == u4.get(3, 0));
  assert!(zero == u4.get(3, 1));
  assert!(zero == u4.get(3, 2));
}
