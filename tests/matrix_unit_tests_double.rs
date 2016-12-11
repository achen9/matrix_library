//! NAME
//!  matrix_unit_tests_double.rs
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
  let mut m: Matrix<f64> = matrix(2, 2);
  m.set(0, 0, 1.6); m.set(0, 1, 2.3);
  m.set(1, 0, 3.2); m.set(1, 1, 4.1);
  assert!(2 == m.rows());
  assert!(2 == m.columns());
  assert!(TOLERANCE > (m.get(0, 0) - 1.6).abs());
  assert!(TOLERANCE > (m.get(0, 1) - 2.3).abs());
  assert!(TOLERANCE > (m.get(1, 0) - 3.2).abs());
  assert!(TOLERANCE > (m.get(1, 1) - 4.1).abs());
}
// Copy Constructor Test: Test if 2x2 matrix can be copied
#[test]
fn copy_constructor_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let mut m1: Matrix<f64> = matrix(2, 2);
  m1.set(0, 0, 1.6); m1.set(0, 1, 2.3);
  m1.set(1, 0, 3.2); m1.set(1, 1, 4.1);
  let mut m: Matrix<f64> = m1.clone();
  assert!(TOLERANCE > (m.get(0, 0) - 1.6).abs());
  assert!(TOLERANCE > (m.get(0, 1) - 2.3).abs());
  assert!(TOLERANCE > (m.get(1, 0) - 3.2).abs());
  assert!(TOLERANCE > (m.get(1, 1) - 4.1).abs());
  m.set(1, 0, 5.23);
  assert!(TOLERANCE > (m.get(1, 0) - 5.23).abs());
  assert!(TOLERANCE > (m1.get(1, 0) - 3.2).abs());
}
// Arithmetic Operation Overload Test 1: 
// Check [1.2 -2.4 + [1.3 -2.4   =  [2.5 -4.8
//        3.6 -9.7]   3.5 -9.6]      7.1 -19.3]
#[test]
fn addition_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let mut m1: Matrix<f64> = matrix(2, 2);
  m1.set(0, 0, 1.2); m1.set(0, 1, -2.4);
  m1.set(1, 0, 3.6); m1.set(1, 1, -9.7);
  let mut m2: Matrix<f64> = matrix(2, 2);
  m2.set(0, 0, 1.3); m2.set(0, 1, -2.4);
  m2.set(1, 0, 3.5); m2.set(1, 1, -9.6);
  let m = m1 + m2;
  assert!(TOLERANCE > (m.get(0, 0) - 2.5).abs());
  assert!(TOLERANCE > (m.get(0, 1) + 4.8).abs());
  assert!(TOLERANCE > (m.get(1, 0) - 7.1).abs());
  assert!(TOLERANCE > (m.get(1, 1) + 19.3).abs());
}
// Arithmetic Operation Overload Test 2: Check 2x2 matrix + 2x3 matrix => panics
#[test]
#[should_panic]
fn addition_error_test() {
  use matrix_lib::matrix::{Matrix, matrix};
 let mut m1: Matrix<f64> = matrix(2, 2);
  m1.set(0, 0, 1.2); m1.set(0, 1, -2.4);
  m1.set(1, 0, 3.6); m1.set(1, 1, -9.7);
  let mut m2: Matrix<f64> = matrix(2, 3);
  m2.set(0, 0, 1.3); m2.set(0, 1, -2.4); m2.set(0, 2, 5.6);
  m2.set(1, 0, 3.5); m2.set(1, 1, -9.6); m2.set(1, 2, -4.5);
  let m = m1 + m2;
  assert!(true); // Something went wrong if this assertion passes
}
// Arithmetic Operation Overload Test 3: 
// Check [1.2 -2.4 - [1.3 -2.4   =  [-0.1 0.0
//        3.6 -9.7]   3.5 -9.6]      0.1 -0.1]
#[test]
fn subtraction_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let mut m1: Matrix<f64> = matrix(2, 2);
  m1.set(0, 0, 1.2); m1.set(0, 1, -2.4);
  m1.set(1, 0, 3.6); m1.set(1, 1, -9.7);
  let mut m2: Matrix<f64> = matrix(2, 2);
  m2.set(0, 0, 1.3); m2.set(0, 1, -2.4);
  m2.set(1, 0, 3.5); m2.set(1, 1, -9.6);
  let m = m1 - m2;
  assert!(TOLERANCE > (m.get(0, 0) + 0.1).abs());
  assert!(TOLERANCE > (m.get(0, 1) - 0.0).abs());
  assert!(TOLERANCE > (m.get(1, 0) - 0.1).abs());
  assert!(TOLERANCE > (m.get(1, 1) + 0.1).abs());
}
// Arithmetic Operation Overload Test 4: Check 2x2 matrix - 2x3 matrix => panics
#[test]
#[should_panic]
fn subtraction_error_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let mut m1: Matrix<f64> = matrix(2, 2);
  m1.set(0, 0, 1.2); m1.set(0, 1, -2.4);
  m1.set(1, 0, 3.6); m1.set(1, 1, -9.7);
  let mut m2: Matrix<f64> = matrix(2, 3);
  m2.set(0, 0, 1.3); m2.set(0, 1, -2.4); m2.set(0, 2, 5.6);
  m2.set(1, 0, 3.5); m2.set(1, 1, -9.6); m2.set(1, 2, -4.5);
  let m = m1 - m2;
  assert!(true); // Something went wrong if this assertion passes
}
// Arithmetic Operation Overload Test 5:
// Check [1.2  * [1.3 -2.4]   =  [1.56 -2.88
//        3.6]                    4.68 -8.64]
#[test]
fn multiplication_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let mut m1: Matrix<f64> = matrix(2, 1);
  m1.set(0, 0, 1.2); m1.set(1, 0, 3.6);
  let mut m2: Matrix<f64> = matrix(1, 2);
  m2.set(0, 0, 1.3); m2.set(0, 1, -2.4);
  let m = m1 * m2;
  assert!(TOLERANCE > (m.get(0, 0) - 1.56).abs());
  assert!(TOLERANCE > (m.get(0, 1) + 2.88).abs());
  assert!(TOLERANCE > (m.get(1, 0) - 4.68).abs());
  assert!(TOLERANCE > (m.get(1, 1) + 8.64).abs());
}
// Arithmetic Operation Overload Test 4: Check 2x1 matrix * 2x1 matrix => panics
#[test]
#[should_panic]
fn multiplication_error_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let mut m1: Matrix<f64> = matrix(2, 1);
  m1.set(0, 0, 1.2); m1.set(1, 0, -2.4);
  let mut m2: Matrix<f64> = matrix(2, 1);
  m2.set(0, 0, 1.3); m2.set(1, 0, -2.4);
  let m = m1 * m2;
  assert!(true); // Something went wrong if this assertion passes
}
/*
// Unary Negate Operator Overload Test : Check -(3.22+4.11j) ~= -3.22-4.11j
#[test]
fn negate_test() {
  use matrix_lib::complex::{Complex, complex};
  let c1: Complex = complex(3.22,4.11);
  let c = -c1;
  assert!(TOLERANCE > (c.re() + 3.22).abs()); 
  assert!(TOLERANCE > (c.im() + 4.11).abs());
  assert!(TOLERANCE > (c1.re() - 3.22).abs()); // Check c1 still exists and can be used
}
// Raising to a Power Test: Check (3+4j)^5 ~= -237-3116j and (1+j)^-2 ~= -j/2
#[test]
fn power_test() {
  use matrix_lib::complex::{Complex, complex};
  let c1: Complex = complex(3.0,4.0);
  let c2: Complex = complex(1.0,1.0);
  let c = c1.pow(5);
  let c3 = c2.pow(-2);
  assert!(TOLERANCE > (c.re() + 237.0).abs());
  assert!(TOLERANCE > (c.im() + 3116.0).abs());
  assert!(TOLERANCE > (c3.re() - 0.0).abs());
  assert!(TOLERANCE > (c3.im() + 0.5).abs());
}
// Raising e to a Complex Power Test: Check (3+4j)^5 ~= -237-3116j and (1+j)^-2 ~= -j/2
#[test]
fn exp_test() {
  use matrix_lib::complex::{Complex, complex};
  let c1: Complex = complex(0.0,PI);
  let c2: Complex = complex(0.0,0.0);
  let c = c1.exp();
  let c3 = c2.exp();
  assert!(TOLERANCE > (c.re() + 1.0).abs());
  assert!(TOLERANCE > (c.im() - 0.0).abs());
  assert!(TOLERANCE > (c3.re() - 1.0).abs());
  assert!(TOLERANCE > (c3.im() - 0.0).abs());
}

// Printing complex to terminal
#[test]
fn print_test() {
  use matrix_lib::complex::{Complex, complex};
  let c1: Complex = complex(-2.011,-4.644);
  let c2: Complex = complex(3.15,0.336);
  println!("C1 is {}. C2 is {}.", c1, c2);
}*/
