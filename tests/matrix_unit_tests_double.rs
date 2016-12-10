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
}/*
// Conjugate method Test: Test if conjugate of 1.22-2.34j => 1.22+2.34j
#[test]
fn conjugate_method_test() {
  use matrix_lib::complex::{Complex, complex};
  let c1: Complex = complex(1.22,-2.34);
  let c2: Complex = c1.conjugate();
  assert!(TOLERANCE > (c1.re() - 1.22).abs());
  assert!(TOLERANCE > (c1.im() + 2.34).abs());
  assert!(TOLERANCE > (c2.re() - 1.22).abs());
  assert!(TOLERANCE > (c2.im() - 2.34).abs());
}
// Arithmetic Operation Overload Test 1: Check 3.22+4.11j + -2.99-3.03j ~= 0.23+1.08j
#[test]
fn addition_test() {
  use matrix_lib::complex::{Complex, complex};
  let c1: Complex = complex(3.22,4.11);
  let c2: Complex = complex(-2.99,-3.03);
  let c = c1 + c2;
  assert!(TOLERANCE > (c.re() - 0.23).abs());
  assert!(TOLERANCE > (c.im() - 1.08).abs());
}
// Arithmetic Operation Overload Test 2: Check 4.25-9.28j - -3.21+6.56j ~= 7.46-15.84j
#[test]
fn subtraction_test() {
  use matrix_lib::complex::{Complex, complex};
  let c1: Complex = complex(4.25,-9.28);
  let c2: Complex = complex(-3.21,6.56);
  let c = c1 - c2;
  assert!(TOLERANCE > (c.re() - 7.46).abs());
  assert!(TOLERANCE > (c.im() + 15.84).abs());
}
// Arithmetic Operation Overload Test 3: Check 2.34-0.0j * 0.0+6.22j ~= 0.0+14.5548
#[test]
fn multiplication_test() {
  use matrix_lib::complex::{Complex, complex};
  let c1: Complex = complex(2.34,-0.0);
  let c2: Complex = complex(0.0,6.22);
  let c = c1 * c2;
  assert!(TOLERANCE > (c.re() - 0.0).abs());
  assert!(TOLERANCE > (c.im() - 14.5548).abs());
}
// Arithmetic Operation Overload Test 4: Check 2.5+3.5j / 1-2j ~= -0.9+1.7j
#[test]
fn division_test1() {
  use matrix_lib::complex::{Complex, complex};
  let c1: Complex = complex(2.5,3.5);
  let c2: Complex = complex(1.0,-2.0);
  let c = c1 / c2;
  assert!(TOLERANCE > (c1.re() - 2.5).abs()); // Check c1 still exists and can be used
  assert!(TOLERANCE > (c2.re() - 1.0).abs()); // Check c2 still exists and can be used
  assert!(TOLERANCE > (c.re() + 0.9).abs());
  assert!(TOLERANCE > (c.im() - 1.7).abs());
}
// Arithmetic Operation Overload Test 5: Check 2.5+3.5j / -0-0j => panic
#[test]
#[should_panic]
fn division_test2() {
  use matrix_lib::complex::{Complex, complex};
  let c1: Complex = complex(2.5,3.5);
  let c2: Complex = complex(-0.0,-0.0);
  let c = c1 / c2;
  assert!(true); // If assertion passes, something went wrong
}
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
