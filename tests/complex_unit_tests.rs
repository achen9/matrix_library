//! NAME
//!  complex_unit_tests.rs
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
//! Alex Z. Chen - 12/09/2016
//! alexac9@uw.edu
//!
extern crate matrix_lib;
// A tolerance constant is needed when comparing doubles. Due to rounding
// errors, doing a straight comparison of doubles may cause erroneous 
// results. Checking the two doubles are within some tolerance is a better
// method to check for equality.
const TOLERANCE: f64 = 0.00001;

// Constructor Test: Test if 2/3 can be assigned to a complex 
#[test]
fn constructor_test() {
  use matrix_lib::complex::{Complex, complex};
  let c: Complex = complex(2.6,3.8);
  assert!(TOLERANCE > (c.re() - 2.6).abs());
  assert!(TOLERANCE > (c.im() - 3.8).abs());
}
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
}
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
  println!("re: {:.5} im: {:.5}",c.re(),c.im());
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
}/*
// Comparison Operator Overload Test 1: Test if 3/-4 == -12/16 => true
#[test]
fn equals_test() {
  use matrix_lib::complex::{Complex, complex};
  let c1: Complex = complex(3,-4);
  let c2: Complex = complex(-12,16);
  assert!(c1 == c2);
}
// Comparison Operator Overload Test 2: Test if 3/-4 != -11/16 => true
#[test]
fn not_equals_test() {
  use matrix_lib::complex::{Complex, complex};
  let c1: Complex = complex(3,-4);
  let c2: Complex = complex(-11,16);
  assert!(c1 != c2);
}
// Comparison Operator Overload Test 3: Test if 3/-4 < 1/2 => true
#[test]
fn less_than_test() {
  use matrix_lib::complex::{Complex, complex};
  let c1: Complex = complex(3,-4);
  let c2: Complex = complex(1,2);
  assert!(c1 < c2);
}
// Comparison Operator Overload Test 4: Test if 1/3 > 3/-4 => true
#[test]
fn greater_than_test() {
  use matrix_lib::complex::{Complex, complex};
  let c1: Complex = complex(3,-4);
  let c2: Complex = complex(1,3);
  assert!(c2 > c1);
}
// Comparison Operator Overload Test 5: Test if 1/3 <= 2/6 => true
#[test]
fn less_than_equals_test() {
  use matrix_lib::complex::{Complex, complex};
  let c1: Complex = complex(1,3);
  let c2: Complex = complex(2,6);
  assert!(c1 <= c2);
  let c3: Complex = complex(-5,1);
  assert!(c3 <= c1);
}
// Comparison Operator Overload Test 6: Test if 1/-8 >= -4/32 => true
#[test]
fn greater_than_equals_test() {
  use matrix_lib::complex::{Complex, complex};
  let c1: Complex = complex(1,-8);
  let c2: Complex = complex(-4,32);
  assert!(c1 >= c2);
  let c3: Complex = complex(1,2);
  assert!(c3 >= c1);
}
// Raising to a Power Test: Test if (2/-4)^3 = -1/8
#[test]
fn power_test() {
  use matrix_lib::complex::{Complex, complex};
  let c1: Complex = complex(2,-4);
  let c2: Complex = c1.pow(3);
  assert!(-1 == c2.re());
  assert!(8 == c2.im());
  let c3: Complex = c1.pow(-3);
  assert!(-8 == c3.re());
  assert!(1 == c3.im());
}
// Printing complex to terminal
#[test]
fn print_test() {
  use matrix_lib::complex::{Complex, complex};
  let c: Complex = complex(2,-4);
  println!("Complex c is {}. c reduced is {}.", c, c.reduce())
}
*/