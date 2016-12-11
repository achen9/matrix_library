//! NAME
//!  matrix_unit_tests_int.rs
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
  let mut m: Matrix<isize> = matrix(2, 2);
  m.set(0, 0, 1); m.set(0, 1, 2);
  m.set(1, 0, 3); m.set(1, 1, 4);
  assert!(2 == m.rows());
  assert!(2 == m.columns());
  assert!(1 == m.get(0, 0));
  assert!(2 == m.get(0, 1));
  assert!(3 == m.get(1, 0));
  assert!(4 == m.get(1, 1));
}
// Copy Constructor Test: Test if 2x2 matrix can be copied
#[test]
fn copy_constructor_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let mut m1: Matrix<isize> = matrix(2, 2);
  m1.set(0, 0, 1); m1.set(0, 1, 2);
  m1.set(1, 0, 3); m1.set(1, 1, 4);
  let mut m: Matrix<isize> = m1.clone();
  assert!(1 == m.get(0, 0));
  assert!(2 == m.get(0, 1));
  assert!(3 == m.get(1, 0));
  assert!(4 == m.get(1, 1));
  m.set(1, 0, 5);
  assert!(5 == m.get(1, 0));
  assert!(3 == m1.get(1, 0));
}
// Arithmetic Operation Overload Test 1: 
// Check [1 -2  + [1 -2   =  [2 -4
//        3 -9]   3 -9]      6 -18]
#[test]
fn addition_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let mut m1: Matrix<isize> = matrix(2, 2);
  m1.set(0, 0, 1); m1.set(0, 1, -2);
  m1.set(1, 0, 3); m1.set(1, 1, -9);
  let mut m2: Matrix<isize> = matrix(2, 2);
  m2.set(0, 0, 1); m2.set(0, 1, -2);
  m2.set(1, 0, 3); m2.set(1, 1, -9);
  let m = m1 + m2;
  assert!(2 == m.get(0, 0));
  assert!(-4 == m.get(0, 1));
  assert!(6 == m.get(1, 0));
  assert!(-18 == m.get(1, 1));
}
// Arithmetic Operation Overload Test 2: Check 2x2 matrix + 2x3 matrix => panics
#[test]
#[should_panic]
fn addition_error_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let mut m1: Matrix<isize> = matrix(2, 2);
  m1.set(0, 0, 1); m1.set(0, 1, -2);
  m1.set(1, 0, 3); m1.set(1, 1, -9);
  let mut m2: Matrix<isize> = matrix(2, 3);
  m2.set(0, 0, 1); m2.set(0, 1, -2); m2.set(0, 2, 5);
  m2.set(1, 0, 3); m2.set(1, 1, -9); m2.set(1, 2, -4);
  let m = m1 + m2;
  assert!(true); // Something went wrong if this assertion passes
}
// Arithmetic Operation Overload Test 3:
// Check [1 -3  - [1 -2   =  [0 -1
//        3 -9]    3 -9]      0 0]
#[test]
fn subtraction_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let mut m1: Matrix<isize> = matrix(2, 2);
  m1.set(0, 0, 1); m1.set(0, 1, -3);
  m1.set(1, 0, 3); m1.set(1, 1, -9);
  let mut m2: Matrix<isize> = matrix(2, 2);
  m2.set(0, 0, 1); m2.set(0, 1, -2);
  m2.set(1, 0, 3); m2.set(1, 1, -9);
  let m = m1 - m2;
  assert!(0 == m.get(0, 0));
  assert!(-1 == m.get(0, 1));
  assert!(0 == m.get(1, 0));
  assert!(0 == m.get(1, 1));
}
// Arithmetic Operation Overload Test 4: Check 2x2 matrix - 2x3 matrix => panics
#[test]
#[should_panic]
fn subtraction_error_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let mut m1: Matrix<isize> = matrix(2, 2);
  m1.set(0, 0, 1); m1.set(0, 1, -2);
  m1.set(1, 0, 3); m1.set(1, 1, -9);
  let mut m2: Matrix<isize> = matrix(2, 3);
  m2.set(0, 0, 1); m2.set(0, 1, -2); m2.set(0, 2, 5);
  m2.set(1, 0, 3); m2.set(1, 1, -9); m2.set(1, 2, -4);
  let m = m1 - m2;
  assert!(true); // Something went wrong if this assertion passes
}
// Arithmetic Operation Overload Test 5: 
// Check [1  * [1 -2]   =  [1 -2
//        3]                3 -6]
#[test]
fn multiplication_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let mut m1: Matrix<isize> = matrix(2, 1);
  m1.set(0, 0, 1); m1.set(1, 0, 3);
  let mut m2: Matrix<isize> = matrix(1, 2);
  m2.set(0, 0, 1); m2.set(0, 1, -2);
  let m = m1 * m2;
  assert!(1 == m.get(0, 0));
  assert!(-2 == m.get(0, 1));
  assert!(3 == m.get(1, 0));
  assert!(-6 == m.get(1, 1));
}
// Arithmetic Operation Overload Test 4: Check 2x1 matrix * 2x1 matrix => panics
#[test]
#[should_panic]
fn multiplication_error_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let mut m1: Matrix<isize> = matrix(2, 1);
  m1.set(0, 0, 1); m1.set(1, 0, -2);
  let mut m2: Matrix<isize> = matrix(2, 1);
  m2.set(0, 0, 1); m2.set(1, 0, -2);
  let m = m1 * m2;
  assert!(true); // Something went wrong if this assertion passes
}
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
