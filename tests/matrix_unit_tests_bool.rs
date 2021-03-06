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
// Identity Method Test: Check a 2x2 identity matrix can be created
#[test]
fn identity_method_test() {
  use matrix_lib::matrix::{Matrix, identity};
  let m: Matrix<bool> = identity(2);
  assert!(true == m.get(0,0));
  assert!(false == m.get(0,1));
  assert!(false == m.get(1,0));
  assert!(true == m.get(1,1));
}
// Ones Method Test: Check a 2x1 ones matrix can be created
#[test]
fn ones_method_test() {
  use matrix_lib::matrix::{Matrix, ones};
  let m: Matrix<bool> = ones(2, 1);
  assert!(true == m.get(0,0));
  assert!(true == m.get(1,0));
}
// Random Method Test: Check a 2x2 random matrix can be created
#[test]
fn random_method_test() {
  use matrix_lib::matrix::{Matrix, random};
  let m: Matrix<bool> = random(2, 2);
  println!("Random matrix m is: {}", m);
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
// Scale Method Test - N/A to boolean types
// Transpose Method Test:
// Check [false]T = [false true]
//       [true]
#[test]
fn transpose_method_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let mut m1: Matrix<bool> = matrix(2, 1);
  m1.set(0, 0, false); m1.set(1, 0, true);
  let m = m1.transpose();
  assert!(1 == m.rows());
  assert!(2 == m.columns());
  assert!(false == m.get(0, 0));
  assert!(true == m.get(0, 1));
}
// Minor Method Test:
// Check [false false false => (1, 1) minor = [false false
//        true  false true                     false false]
//        false true  false]
#[test]
fn minor_method_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let mut m1: Matrix<bool> = matrix(3, 3);
  m1.set(0, 0, false); m1.set(0, 1, false); m1.set(0, 2, false);
  m1.set(1, 0, true); m1.set(1, 1, false); m1.set(1, 2, true);
  m1.set(2, 0, false); m1.set(2, 1, true); m1.set(2, 2, false);
  let m = m1.minor(1, 1);
  assert!(2 == m.rows());
  assert!(2 == m.columns());
  assert!(false == m.get(0, 0));
  assert!(false == m.get(0, 1));
  assert!(false == m.get(1, 0));
  assert!(false == m.get(1, 1));
}
// Determinant Method Test - N/A to boolean types
// Printing Matrix to Terminal
#[test]
fn print_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let mut m: Matrix<bool> = matrix(2, 2);
  m.set(0, 0, true); m.set(0, 1, true);
  m.set(1, 0, false); m.set(1, 1, false);
  println!("M is: {}", m);
}
