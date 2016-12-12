//! NAME
//!  matrix_unit_tests_complex.rs
//!
//! DESCRIPTION
//!  Unit test file for complex class methods
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
//! Alex Z. Chen - 12/10/2016
//! alexac9@uw.edu
//!
extern crate matrix_lib;
// A tolerance constant is needed when comparing doubles. Due to rounding
// errors, doing a straight comparison of doubles may cause erroneous 
// results. Checking the two doubles are within some tolerance is a better
// method to check for equality.
const TOLERANCE: f64 = 0.00001;

// Constructor Test: Test if 2x2 matrix can be created and values assigned 
#[test]
fn constructor_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::complex::{Complex, complex};
  let mut m: Matrix<Complex> = matrix(2, 2);
  let c1 = complex(1.2, 3.1); let c2 = complex(-2.4, 3.9);
  let c3 = complex(3.6, 4.0); let c4 = complex(-9.7, 5.4);
  m.set(0, 0, c1); m.set(0, 1, c2);
  m.set(1, 0, c3); m.set(1, 1, c4);
  assert!(TOLERANCE > (m.get(0, 0).re() - c1.re()).abs() && TOLERANCE > (m.get(0, 0).im() - c1.im()).abs());
  assert!(TOLERANCE > (m.get(0, 1).re() - c2.re()).abs() && TOLERANCE > (m.get(0, 1).im() - c2.im()).abs());
  assert!(TOLERANCE > (m.get(1, 0).re() - c3.re()).abs() && TOLERANCE > (m.get(1, 0).im() - c3.im()).abs());
  assert!(TOLERANCE > (m.get(1, 1).re() - c4.re()).abs() && TOLERANCE > (m.get(1, 1).im() - c4.im()).abs());
}
// Copy Constructor Test: Test if 2x2 matrix can be copied
#[test]
fn copy_constructor_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::complex::{Complex, complex};
  let mut m1: Matrix<Complex> = matrix(2, 2);
  let c1 = complex(1.2, 3.1); let c2 = complex(-2.4, 3.9);
  let c3 = complex(3.6, 4.0); let c4 = complex(-9.7, 5.4);
  m1.set(0, 0, c1); m1.set(0, 1, c2);
  m1.set(1, 0, c3); m1.set(1, 1, c4);
  let mut m: Matrix<Complex> = m1.clone();
  assert!(TOLERANCE > (m.get(0, 0).re() - c1.re()).abs() && TOLERANCE > (m.get(0, 0).im() - c1.im()).abs());
  assert!(TOLERANCE > (m.get(0, 1).re() - c2.re()).abs() && TOLERANCE > (m.get(0, 1).im() - c2.im()).abs());
  assert!(TOLERANCE > (m.get(1, 0).re() - c3.re()).abs() && TOLERANCE > (m.get(1, 0).im() - c3.im()).abs());
  assert!(TOLERANCE > (m.get(1, 1).re() - c4.re()).abs() && TOLERANCE > (m.get(1, 1).im() - c4.im()).abs());
  let c5 = complex(3.66, -9.55);
  m.set(1, 0, c5);
  assert!(TOLERANCE > (m.get(1, 0).re() - c5.re()).abs() && TOLERANCE > (m.get(1, 0).im() - c5.im()).abs());
  assert!(TOLERANCE > (m1.get(1, 0).re() - c3.re()).abs() && TOLERANCE > (m1.get(1, 0).im() - c3.im()).abs());
}
// Arithmetic Operation Overload Test 1:
// Check [1.2+3.1j -2.4+3.9j  + [1.3+3.0j -2.4+3.8j   =  [2.5+6.1j -4.8+7.7j
//        3.6+4.0j -9.7+5.4j]    3.5+3.9j -9.6+5.3j]      7.1+7.9j -19.3+10.7j]
#[test]
fn addition_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::complex::{Complex, complex};
  let mut m1: Matrix<Complex> = matrix(2, 2);
  let m11 = complex(1.2, 3.1); let m12 = complex(-2.4, 3.9);
  let m13 = complex(3.6, 4.0); let m14 = complex(-9.7, 5.4);
  m1.set(0, 0, m11); m1.set(0, 1, m12);
  m1.set(1, 0, m13); m1.set(1, 1, m14);
  let mut m2: Matrix<Complex> = matrix(2, 2);
  let m21 = complex(1.3, 3.0); let m22 = complex(-2.4, 3.8);
  let m23 = complex(3.5, 3.9); let m24 = complex(-9.6, 5.3);
  m2.set(0, 0, m21); m2.set(0, 1, m22);
  m2.set(1, 0, m23); m2.set(1, 1, m24);
  let m = m1 + m2;
  assert!(TOLERANCE > (m.get(0, 0).re() - 2.5).abs() && TOLERANCE > (m.get(0, 0).im() - 6.1).abs());
  assert!(TOLERANCE > (m.get(0, 1).re() + 4.8).abs() && TOLERANCE > (m.get(0, 1).im() - 7.7).abs());
  assert!(TOLERANCE > (m.get(1, 0).re() - 7.1).abs() && TOLERANCE > (m.get(1, 0).im() - 7.9).abs());
  assert!(TOLERANCE > (m.get(1, 1).re() + 19.3).abs() && TOLERANCE > (m.get(1, 1).im() - 10.7).abs());
}
// Arithmetic Operation Overload Test 2: Check 2x2 matrix + 2x3 matrix => panics
#[test]
#[should_panic]
fn addition_error_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::complex::{Complex, complex};
  let mut m1: Matrix<Complex> = matrix(2, 2);
  let m11 = complex(1.2, 3.1); let m12 = complex(-2.4, 3.9);
  let m13 = complex(3.6, 4.0); let m14 = complex(-9.7, 5.4);
  m1.set(0, 0, m11); m1.set(0, 1, m12);
  m1.set(1, 0, m13); m1.set(1, 1, m14);
  let mut m2: Matrix<Complex> = matrix(2, 3);
  let m21 = complex(1.3, 3.0); let m22 = complex(-2.4, 3.8); let m23 = complex(3.5, 3.9); 
  let m24 = complex(-9.6, 5.3); let m25 = complex(2.1, 1.2); let m26 = complex(3.3, 4.5);
  m2.set(0, 0, m21); m2.set(0, 1, m22); m2.set(0, 2, m23); 
  m2.set(1, 0, m24); m2.set(1, 1, m25); m2.set(1, 2, m26);
  let m = m1 + m2;
  assert!(true); // Something went wrong if this assertion passes
}
// Arithmetic Operation Overload Test 3
// Check [1.2+3.1j -2.4+3.9j  - [1.3+3.0j -2.4+3.8j   =  [-0.1+0.1j 0.0+0.1j
//        3.6+4.0j -9.7+5.4j]    3.5+3.9j -9.6+5.3j]      0.1+0.1j -0.1+0.1j]
#[test]
fn subtraction_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::complex::{Complex, complex};
  let mut m1: Matrix<Complex> = matrix(2, 2);
  let m11 = complex(1.2, 3.1); let m12 = complex(-2.4, 3.9);
  let m13 = complex(3.6, 4.0); let m14 = complex(-9.7, 5.4);
  m1.set(0, 0, m11); m1.set(0, 1, m12);
  m1.set(1, 0, m13); m1.set(1, 1, m14);
  let mut m2: Matrix<Complex> = matrix(2, 2);
  let m21 = complex(1.3, 3.0); let m22 = complex(-2.4, 3.8);
  let m23 = complex(3.5, 3.9); let m24 = complex(-9.6, 5.3);
  m2.set(0, 0, m21); m2.set(0, 1, m22);
  m2.set(1, 0, m23); m2.set(1, 1, m24);
  let m = m1 - m2;
  assert!(TOLERANCE > (m.get(0, 0).re() + 0.1).abs() && TOLERANCE > (m.get(0, 0).im() - 0.1).abs());
  assert!(TOLERANCE > (m.get(0, 1).re() - 0.0).abs() && TOLERANCE > (m.get(0, 1).im() - 0.1).abs());
  assert!(TOLERANCE > (m.get(1, 0).re() - 0.1).abs() && TOLERANCE > (m.get(1, 0).im() - 0.1).abs());
  assert!(TOLERANCE > (m.get(1, 1).re() + 0.1).abs() && TOLERANCE > (m.get(1, 1).im() - 0.1).abs());
}
// Arithmetic Operation Overload Test 4: Check 2x2 matrix - 2x3 matrix => panics
#[test]
#[should_panic]
fn subtraction_error_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::complex::{Complex, complex};
  let mut m1: Matrix<Complex> = matrix(2, 2);
  let m11 = complex(1.2, 3.1); let m12 = complex(-2.4, 3.9);
  let m13 = complex(3.6, 4.0); let m14 = complex(-9.7, 5.4);
  m1.set(0, 0, m11); m1.set(0, 1, m12);
  m1.set(1, 0, m13); m1.set(1, 1, m14);
  let mut m2: Matrix<Complex> = matrix(2, 3);
  let m21 = complex(1.3, 3.0); let m22 = complex(-2.4, 3.8); let m23 = complex(3.5, 3.9); 
  let m24 = complex(-9.6, 5.3); let m25 = complex(2.1, 1.2); let m26 = complex(3.3, 4.5);
  m2.set(0, 0, m21); m2.set(0, 1, m22); m2.set(0, 2, m23); 
  m2.set(1, 0, m24); m2.set(1, 1, m25); m2.set(1, 2, m26);
  let m = m1 - m2;
  assert!(true); // Something went wrong if this assertion passes
}
// Arithmetic Operation Overload Test 5:
// Check [1.2+3.1j  * [1.3+3.0j -2.4+3.8j]   =  [-7.74+7.63j -14.66-2.88j
//        3.6+4.0j]                              -7.32+16.0j -23.84+4.08j]
#[test]
fn multiplication_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::complex::{Complex, complex};
  let mut m1: Matrix<Complex> = matrix(2, 1);
  let m11 = complex(1.2, 3.1); let m12 = complex(3.6, 4.0);
  m1.set(0, 0, m11); m1.set(1, 0, m12);
  let mut m2: Matrix<Complex> = matrix(1, 2);
  let m21 = complex(1.3, 3.0); let m22 = complex(-2.4, 3.8);
  m2.set(0, 0, m21); m2.set(0, 1, m22);
  let m = m1 * m2;
  assert!(TOLERANCE > (m.get(0, 0).re() + 7.74).abs() && TOLERANCE > (m.get(0, 0).im() - 7.63).abs());
  assert!(TOLERANCE > (m.get(0, 1).re() + 14.66).abs() && TOLERANCE > (m.get(0, 1).im() + 2.88).abs());
  assert!(TOLERANCE > (m.get(1, 0).re() + 7.32).abs() && TOLERANCE > (m.get(1, 0).im() - 16.0).abs());
  assert!(TOLERANCE > (m.get(1, 1).re() + 23.84).abs() && TOLERANCE > (m.get(1, 1).im() - 4.08).abs());
}
// Arithmetic Operation Overload Test 6: Check 2x1 matrix * 2x1 matrix => panics
#[test]
#[should_panic]
fn multiplication_error_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::complex::{Complex, complex};
  let mut m1: Matrix<Complex> = matrix(2, 1);
  let m11 = complex(1.2, 3.1); let m12 = complex(3.6, 4.0); 
  m1.set(0, 0, m11); m1.set(1, 0, m12);
  let mut m2: Matrix<Complex> = matrix(2, 1);
  let m21 = complex(1.3, 3.0); let m22 = complex(-2.4, 3.8);  
  m2.set(0, 0, m21); m2.set(1, 0, m22); 
  let m = m1 * m2;
  assert!(true); // Something went wrong if this assertion passes
}
// Scale Method Test:
// Check [1.2+3.1j -2.4+3.9j   * 1.0+1.0j =  [-1.9+4.3j -6.3+1.5j
//        3.6+4.0j -9.7+5.4j]                 -0.4+7.6j -15.1-4.3j]
#[test]
fn scale_method_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::complex::{Complex, complex};
  let mut m1: Matrix<Complex> = matrix(2, 2);
  let m11 = complex(1.2, 3.1); let m12 = complex(-2.4, 3.9);
  let m13 = complex(3.6, 4.0); let m14 = complex(-9.7, 5.4);
  m1.set(0, 0, m11); m1.set(0, 1, m12);
  m1.set(1, 0, m13); m1.set(1, 1, m14);
  let s = complex(1.0, 1.0);
  let m = m1.scale(s);
  assert!(TOLERANCE > (m.get(0, 0).re() + 1.9).abs() && TOLERANCE > (m.get(0, 0).im() - 4.3).abs());
  assert!(TOLERANCE > (m.get(0, 1).re() + 6.3).abs() && TOLERANCE > (m.get(0, 1).im() - 1.5).abs());
  assert!(TOLERANCE > (m.get(1, 0).re() + 0.4).abs() && TOLERANCE > (m.get(1, 0).im() - 7.6).abs());
  assert!(TOLERANCE > (m.get(1, 1).re() + 15.1).abs() && TOLERANCE > (m.get(1, 1).im() + 4.3).abs());
}
// Transpose Method Test:
// Check [1.2+3.1j]T = [1.2+3.1j 3.6+4.0j] 
//        3.6+4.0j]           
#[test]
fn transpose_method_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::complex::{Complex, complex};
  let mut m1: Matrix<Complex> = matrix(2, 1);
  let m11 = complex(1.2, 3.1); let m12 = complex(3.6, 4.0);
  m1.set(0, 0, m11); m1.set(1, 0, m12);
  let m = m1.transpose();
  assert!(1 == m.rows());
  assert!(2 == m.columns());
  assert!(TOLERANCE > (m.get(0, 0).re() - 1.2).abs() && TOLERANCE > (m.get(0, 0).im() - 3.1).abs());
  assert!(TOLERANCE > (m.get(0, 1).re() - 3.6).abs() && TOLERANCE > (m.get(0, 1).im() - 4.0).abs());
}
// Minor Method Test:
// Check [1.2+3.1j 1.3-6.5j -2.4+3.9j  => (1, 1) minor => [1.2+3.1j -2.4+3.9j
//        2.6+9.5j 2.1-5.1j 3.1+1.5j                       3.6+4.0j -9.7+5.4j]                   
//        3.6+4.0j 3.2+5.9j -9.7+5.4j]                 
#[test]
fn minor_method_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::complex::{Complex, complex};
  let mut m1: Matrix<Complex> = matrix(3, 3);
  let m11 = complex(1.2, 3.1); let m12 = complex(1.3, -6.5); let m13 = complex(-2.4, 3.9);
  let m14 = complex(2.6, 9.5); let m15 = complex(2.1, -5.1); let m16 = complex(3.1, 1.5);
  let m17 = complex(3.6, 4.0); let m18 = complex(3.2, 5.9); let m19 = complex(-9.7, 5.4);
  m1.set(0, 0, m11); m1.set(0, 1, m12); m1.set(0, 2, m13);
  m1.set(1, 0, m14); m1.set(1, 1, m15); m1.set(1, 2, m16);
  m1.set(2, 0, m17); m1.set(2, 1, m18); m1.set(2, 2, m19);
  let m = m1.minor(1, 1);
  assert!(2 == m.rows());
  assert!(2 == m.columns());
  assert!(m11 == m.get(0, 0));
  assert!(m13 == m.get(0, 1));
  assert!(m17 == m.get(1, 0));
  assert!(m19 == m.get(1, 1));
}
// Determinant Method Test:
// Check determinant [5.0+0.0j 3.0+0.0j -4.0+0.0j   = 4.0+0.0j
//                    2.0+0.0j 0.0+0.0j -2.0+0.0j                                     
//                    2.0+0.0j 5.0+0.0j -1.0+0.0j]                 
#[test]
fn determinant_method_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::complex::{Complex, complex};
  let mut m1: Matrix<Complex> = matrix(3, 3);
  let m11 = complex(5.0, 0.0); let m12 = complex(3.0, 0.0); let m13 = complex(-4.0, 0.0);
  let m14 = complex(2.0, 0.0); let m15 = complex(0.0, 0.0); let m16 = complex(-2.0, 0.0);
  let m17 = complex(2.0, 0.0); let m18 = complex(5.0, 0.0); let m19 = complex(-1.0, 0.0);
  m1.set(0, 0, m11); m1.set(0, 1, m12); m1.set(0, 2, m13);
  m1.set(1, 0, m14); m1.set(1, 1, m15); m1.set(1, 2, m16);
  m1.set(2, 0, m17); m1.set(2, 1, m18); m1.set(2, 2, m19);
  let m = m1.det();
  let c = complex(4.0, 0.0);
  assert!(c == m);
}
/*
// Printing complex to terminal
#[test]
fn print_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let m1: Matrix<isize> = matrix<isize>(-2.011,-4.644);
  let m2: Matrix<isize> = matrix<isize>(3.15,0.336);
  println!("C1 is {}. C2 is {}.", m1, m2);
}*/
