//! NAME
//!  matrix_unit_tests_bool.rs
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

// Constructor Test: Test if 2x2 matrix can be created and values assigned 
#[test]
fn constructor_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let mut m: Matrix<bool> = matrix(2, 2);
  m.set(0, 0, true); m.set(0, 1, true);
  m.set(1, 0, false); m.set(1, 1, false);
  assert!(2 == m.rows());
  assert!(2 == m.columns());
  assert!(true == m.get(0, 0));
  assert!(true == m.get(0, 1));
  assert!(false == m.get(1, 0));
  assert!(false == m.get(1, 1));
}
// Copy Constructor Test: Test if 2x2 matrix can be copied
#[test]
fn copy_constructor_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let mut m1: Matrix<bool> = matrix(2, 2);
  m1.set(0, 0, true); m1.set(0, 1, true);
  m1.set(1, 0, false); m1.set(1, 1, false);
  let mut m: Matrix<bool> = m1.clone();
  assert!(true == m.get(0, 0));
  assert!(true == m.get(0, 1));
  assert!(false == m.get(1, 0));
  assert!(false == m.get(1, 1));
  m.set(1, 0, true);
  assert!(true == m.get(1, 0));
  assert!(false == m1.get(1, 0));
}
// Arithmetic Operation Overload Tests - N/A to boolean types
/*
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
