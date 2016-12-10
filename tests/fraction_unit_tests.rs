//! NAME
//!  fraction_unit_tests.rs
//!
//! DESCRIPTION
//!  Unit test file for fraction class methods
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
//! Alex Z. Chen - 12/05/2016
//! alexac9@uw.edu
//!
extern crate matrix_lib;

// Constructor Test 1: Test if 2/3 can be assigned to a fraction 
#[test]
fn constructor_test1() {
  use matrix_lib::fraction::{Fraction, fraction};
  let f: Fraction = fraction(2,3);
  assert!(2 == f.num());
  assert!(3 == f.den());
  
}
// Constructor Test 2: Test if setting denominator to 0 will cause panic
#[test]
#[should_panic]
fn constructor_test2() {
  use matrix_lib::fraction::{Fraction, fraction};
  let f1: Fraction = fraction(2,0);
  assert!(2 == f1.num()); // If it passes this assert, something went wrong
}
// Copy Constructor Test: Test if fraction -3/4 can be copied to another variable
#[test]
fn copy_constructor_test() {
  use matrix_lib::fraction::{Fraction, fraction};
  let f1: Fraction = fraction(-3,4);
  let mut f2: Fraction = f1.clone();
  assert!(-3 == f2.num());
  assert!(4 == f2.den());
  f2.set_num(5);
  assert!(5 == f2.num());
  assert!(-3 == f1.num());
}
// Reduce method Test: Test if fraction 8/-32 can be reduced to -1/4
#[test]
fn reduce_method_test() {
  use matrix_lib::fraction::{Fraction, fraction};
  let f1: Fraction = fraction(8,-32);
  let f2: Fraction = f1.reduce();
  assert!(-1 == f2.num());
  assert!(4 == f2.den());
  assert!(8 == f1.num());
  assert!(-32 == f1.den());
}
// Arithmetic Operation Overload Test 1: Test if 3/4 + 2/-3 = 1/12
#[test]
fn addition_test() {
  use matrix_lib::fraction::{Fraction, fraction};
  let f1: Fraction = fraction(3,4);
  let f2: Fraction = fraction(2,-3);
  let f = f1 + f2;
  assert!(1 == f.num());
  assert!(12 == f.den());
}
// Arithmetic Operation Overload Test 2: Test if 2/4 - 2/3 = -1/6
#[test]
fn subtraction_test() {
  use matrix_lib::fraction::{Fraction, fraction};
  let f1: Fraction = fraction(2,4);
  let f2: Fraction = fraction(2,3);
  let f = f1 - f2;
  assert!(-1 == f.num());
  assert!(6 == f.den());
}
// Arithmetic Operation Overload Test 3: Test if 4/-5 * 3/4 = -3/5
#[test]
fn multiplication_test() {
  use matrix_lib::fraction::{Fraction, fraction};
  let f1: Fraction = fraction(4,-5);
  let f2: Fraction = fraction(3,4);
  let f = f1 * f2;
  assert!(-3 == f.num());
  assert!(5 == f.den());
}
// Arithmetic Operation Overload Test 4: Test if (-5/7) / (-6/7) = 5/6
#[test]
fn division_test1() {
  use matrix_lib::fraction::{Fraction, fraction};
  let f1: Fraction = fraction(-5,7);
  let f2: Fraction = fraction(-6,7);
  let f = f1 / f2;
  assert!(-5 == f1.num()); // Check f1 still exists and can be used
  assert!(-6 == f2.num()); // Check f2 still exists and can be used
  assert!(5 == f.num());
  assert!(6 == f.den());
}
// Arithmetic Operation Overload Test 4: Test if (-5/7) / (0/2) = panic
#[test]
#[should_panic]
fn division_test2() {
  use matrix_lib::fraction::{Fraction, fraction};
  let f1: Fraction = fraction(-5,7);
  let f2: Fraction = fraction(0,2);
  let f = f1 / f2;
  assert!(true); // If assertion passes, something went wrong
}
// Unary Negate Operator Overload Test : Test if -(-5/-7) = -5/7
#[test]
fn negate_test() {
  use matrix_lib::fraction::{Fraction, fraction};
  let f1: Fraction = fraction(-5,-7);
  let f = -f1;
  assert!(-5 == f.num()); 
  assert!(7 == f.den());
  assert!(-5 == f1.num()); // Check f1 still exists and can be used
}
// Comparison Operator Overload Test 1: Test if 3/-4 == -12/16 => true
#[test]
fn equals_test() {
  use matrix_lib::fraction::{Fraction, fraction};
  let f1: Fraction = fraction(3,-4);
  let f2: Fraction = fraction(-12,16);
  assert!(f1 == f2);
}
// Comparison Operator Overload Test 2: Test if 3/-4 != -11/16 => true
#[test]
fn not_equals_test() {
  use matrix_lib::fraction::{Fraction, fraction};
  let f1: Fraction = fraction(3,-4);
  let f2: Fraction = fraction(-11,16);
  assert!(f1 != f2);
}
// Comparison Operator Overload Test 3: Test if 3/-4 < 1/2 => true
#[test]
fn less_than_test() {
  use matrix_lib::fraction::{Fraction, fraction};
  let f1: Fraction = fraction(3,-4);
  let f2: Fraction = fraction(1,2);
  assert!(f1 < f2);
}
// Comparison Operator Overload Test 4: Test if 1/3 > 3/-4 => true
#[test]
fn greater_than_test() {
  use matrix_lib::fraction::{Fraction, fraction};
  let f1: Fraction = fraction(3,-4);
  let f2: Fraction = fraction(1,3);
  assert!(f2 > f1);
}
// Comparison Operator Overload Test 5: Test if 1/3 <= 2/6 => true
#[test]
fn less_than_equals_test() {
  use matrix_lib::fraction::{Fraction, fraction};
  let f1: Fraction = fraction(1,3);
  let f2: Fraction = fraction(2,6);
  assert!(f1 <= f2);
  let f3: Fraction = fraction(-5,1);
  assert!(f3 <= f1);
}
// Comparison Operator Overload Test 6: Test if 1/-8 >= -4/32 => true
#[test]
fn greater_than_equals_test() {
  use matrix_lib::fraction::{Fraction, fraction};
  let f1: Fraction = fraction(1,-8);
  let f2: Fraction = fraction(-4,32);
  assert!(f1 >= f2);
  let f3: Fraction = fraction(1,2);
  assert!(f3 >= f1);
}
// Raising to a Power Test: Test if (2/-4)^3 = -1/8
#[test]
fn power_test() {
  use matrix_lib::fraction::{Fraction, fraction};
  let f1: Fraction = fraction(2,-4);
  let f2: Fraction = f1.pow(3);
  assert!(-1 == f2.num());
  assert!(8 == f2.den());
  let f3: Fraction = f1.pow(-3);
  assert!(-8 == f3.num());
  assert!(1 == f3.den());
}
// Printing fraction to terminal
#[test]
fn print_test() {
  use matrix_lib::fraction::{Fraction, fraction};
  let f: Fraction = fraction(2,-4);
  println!("Fraction f is {}. f reduced is {}.", f, f.reduce())
}
