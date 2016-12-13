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
// Unitary Matrix Method Test: Spot check unitary matrix elements are 1+0j
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
