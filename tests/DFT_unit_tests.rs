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
}/*
// Copy Constructor Test: Test if complex -3/4 can be copied to another variable
#[test]
fn copy_constructor_test() {
  use matrix_lib::complex::{Complex, complex};
  let c1: Complex = complex(-3.95,4.12);
  let mut c2: Complex = c1.clone();
  assert!(TOLERANCE > (c2.re() + 3.95).abs());
  assert!(TOLERANCE > (c2.im() - 4.12).abs());
  c2.set_re(5.23);
  assert!(TOLERANCE > (c2.re() - 5.23).abs());
  assert!(TOLERANCE > (c1.re() + 3.95).abs());
}*/

