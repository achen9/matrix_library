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
// Scale method test - N/A to boolean types

/*
// Printing complex to terminal
#[test]
fn print_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let m1: Matrix<isize> = matrix<isize>(-2.011,-4.644);
  let m2: Matrix<isize> = matrix<isize>(3.15,0.336);
  println!("C1 is {}. C2 is {}.", m1, m2);
}*/
