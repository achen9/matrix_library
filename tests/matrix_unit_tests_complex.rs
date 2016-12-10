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
}/*
// Copy Constructor Test: Test if matrix<isize> -3/4 can be copied to another variable
#[test]
fn copy_constructor_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let m1: Matrix<isize> = matrix<isize>(-3.95,4.12);
  let mut m2: Matrix<isize> = m1.clone();
  assert!(TOLERANCE > (m2.re() + 3.95).abs());
  assert!(TOLERANCE > (m2.im() - 4.12).abs());
  m2.set_re(5.23);
  assert!(TOLERANCE > (m2.re() - 5.23).abs());
  assert!(TOLERANCE > (m1.re() + 3.95).abs());
}
// Conjugate method Test: Test if conjugate of 1.22-2.34j => 1.22+2.34j
#[test]
fn conjugate_method_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let m1: Matrix<isize> = matrix<isize>(1.22,-2.34);
  let m2: Matrix<isize> = m1.conjugate();
  assert!(TOLERANCE > (m1.re() - 1.22).abs());
  assert!(TOLERANCE > (m1.im() + 2.34).abs());
  assert!(TOLERANCE > (m2.re() - 1.22).abs());
  assert!(TOLERANCE > (m2.im() - 2.34).abs());
}
// Arithmetic Operation Overload Test 1: Check 3.22+4.11j + -2.99-3.03j ~= 0.23+1.08j
#[test]
fn addition_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let m1: Matrix<isize> = matrix<isize>(3.22,4.11);
  let m2: Matrix<isize> = matrix<isize>(-2.99,-3.03);
  let m = m1 + m2;
  assert!(TOLERANCE > (m.re() - 0.23).abs());
  assert!(TOLERANCE > (m.im() - 1.08).abs());
}
// Arithmetic Operation Overload Test 2: Check 4.25-9.28j - -3.21+6.56j ~= 7.46-15.84j
#[test]
fn subtraction_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let m1: Matrix<isize> = matrix<isize>(4.25,-9.28);
  let m2: Matrix<isize> = matrix<isize>(-3.21,6.56);
  let m = m1 - m2;
  assert!(TOLERANCE > (m.re() - 7.46).abs());
  assert!(TOLERANCE > (m.im() + 15.84).abs());
}
// Arithmetic Operation Overload Test 3: Check 2.34-0.0j * 0.0+6.22j ~= 0.0+14.5548
#[test]
fn multiplication_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let m1: Matrix<isize> = matrix<isize>(2.34,-0.0);
  let m2: Matrix<isize> = matrix<isize>(0.0,6.22);
  let m = m1 * m2;
  assert!(TOLERANCE > (m.re() - 0.0).abs());
  assert!(TOLERANCE > (m.im() - 14.5548).abs());
}
// Arithmetic Operation Overload Test 4: Check 2.5+3.5j / 1-2j ~= -0.9+1.7j
#[test]
fn division_test1() {
  use matrix_lib::matrix::{Matrix, matrix};
  let m1: Matrix<isize> = matrix<isize>(2.5,3.5);
  let m2: Matrix<isize> = matrix<isize>(1.0,-2.0);
  let m = m1 / m2;
  assert!(TOLERANCE > (m1.re() - 2.5).abs()); // Check c1 still exists and can be used
  assert!(TOLERANCE > (m2.re() - 1.0).abs()); // Check c2 still exists and can be used
  assert!(TOLERANCE > (m.re() + 0.9).abs());
  assert!(TOLERANCE > (m.im() - 1.7).abs());
}
// Arithmetic Operation Overload Test 5: Check 2.5+3.5j / -0-0j => panic
#[test]
#[should_panic]
fn division_test2() {
  use matrix_lib::matrix::{Matrix, matrix};
  let m1: Matrix<isize> = matrix<isize>(2.5,3.5);
  let m2: Matrix<isize> = matrix<isize>(-0.0,-0.0);
  let m = m1 / m2;
  assert!(true); // If assertion passes, something went wrong
}
// Unary Negate Operator Overload Test : Check -(3.22+4.11j) ~= -3.22-4.11j
#[test]
fn negate_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let m1: Matrix<isize> = matrix<isize>(3.22,4.11);
  let m = -m1;
  assert!(TOLERANCE > (m.re() + 3.22).abs()); 
  assert!(TOLERANCE > (m.im() + 4.11).abs());
  assert!(TOLERANCE > (m1.re() - 3.22).abs()); // Check c1 still exists and can be used
}
// Raising to a Power Test: Check (3+4j)^5 ~= -237-3116j and (1+j)^-2 ~= -j/2
#[test]
fn power_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let m1: Matrix<isize> = matrix<isize>(3.0,4.0);
  let m2: Matrix<isize> = matrix<isize>(1.0,1.0);
  let m = m1.pow(5);
  let m3 = m2.pow(-2);
  assert!(TOLERANCE > (m.re() + 237.0).abs());
  assert!(TOLERANCE > (m.im() + 3116.0).abs());
  assert!(TOLERANCE > (m3.re() - 0.0).abs());
  assert!(TOLERANCE > (m3.im() + 0.5).abs());
}
// Raising e to a Matrix<isize> Power Test: Check (3+4j)^5 ~= -237-3116j and (1+j)^-2 ~= -j/2
#[test]
fn exp_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let m1: Matrix<isize> = matrix<isize>(0.0,PI);
  let m2: Matrix<isize> = matrix<isize>(0.0,0.0);
  let m = m1.exp();
  let m3 = m2.exp();
  assert!(TOLERANCE > (m.re() + 1.0).abs());
  assert!(TOLERANCE > (m.im() - 0.0).abs());
  assert!(TOLERANCE > (m3.re() - 1.0).abs());
  assert!(TOLERANCE > (m3.im() - 0.0).abs());
}

// Printing complex to terminal
#[test]
fn print_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let m1: Matrix<isize> = matrix<isize>(-2.011,-4.644);
  let m2: Matrix<isize> = matrix<isize>(3.15,0.336);
  println!("C1 is {}. C2 is {}.", m1, m2);
}*/
