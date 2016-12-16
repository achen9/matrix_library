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
// Identity Method Test: Check a 2x2 identity matrix can be created
#[test]
fn identity_method_test() {
  use matrix_lib::matrix::{Matrix, identity};
  let m: Matrix<isize> = identity(2);
  assert!(1 == m.get(0, 0));
  assert!(0 == m.get(0, 1));
  assert!(0 == m.get(1, 0));
  assert!(1 == m.get(1, 1));
}
// Ones Method Test: Check a 2x1 ones matrix can be created
#[test]
fn ones_method_test() {
  use matrix_lib::matrix::{Matrix, ones};
  let m: Matrix<isize> = ones(2, 1);
  assert!(1 == m.get(0, 0));
  assert!(1 == m.get(1, 0));
}
// Random Method Test: Check a 2x2 random matrix can be created
#[test]
fn random_method_test() {
  use matrix_lib::matrix::{Matrix, random};
  let m: Matrix<isize> = random(2, 2);
  println!("Random matrix m is: {}", m);
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
  assert!(m == m); // Something went wrong if this assertion passes
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
  assert!(m == m); // Something went wrong if this assertion passes
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
  assert!(m == m); // Something went wrong if this assertion passes
}
// Scale Method Test:
// Check [1 -3  * 4 =  [4 -12
//        3 -9]         12 -36]
#[test]
fn scale_method_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let mut m1: Matrix<isize> = matrix(2, 2);
  m1.set(0, 0, 1); m1.set(0, 1, -3);
  m1.set(1, 0, 3); m1.set(1, 1, -9);
  let s = 4;
  let m = m1.scale(s);
  assert!(4 == m.get(0, 0));
  assert!(-12 == m.get(0, 1));
  assert!(12 == m.get(1, 0));
  assert!(-36 == m.get(1, 1));
}
// Transpose Method Test:
// Check [1]T =  [1 3]
//       [3]
#[test]
fn transpose_method_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let mut m1: Matrix<isize> = matrix(2, 1);
  m1.set(0, 0, 1); m1.set(1, 0, 3);
  let m = m1.transpose();
  assert!(1 == m.rows());
  assert!(2 == m.columns());
  assert!(1 == m.get(0, 0));
  assert!(3 == m.get(0, 1));

}
// Minor Method Test:
// Check [1 2 -3  => (1, 1) minor =>  [1 -3
//        6 -9 8                       3 -9]
//        3 7 -9]
#[test]
fn minor_method_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let mut m1: Matrix<isize> = matrix(3, 3);
  m1.set(0, 0, 1); m1.set(0, 1, 2); m1.set(0, 2, -3);
  m1.set(1, 0, 6); m1.set(1, 1, -9); m1.set(1, 2, 8);
  m1.set(2, 0, 3); m1.set(2, 1, 7); m1.set(2, 2, -9);
  let m = m1.minor(1, 1);
  assert!(2 == m.rows());
  assert!(2 == m.columns());
  assert!(1 == m.get(0, 0));
  assert!(-3 == m.get(0, 1));
  assert!(3 == m.get(1, 0));
  assert!(-9 == m.get(1, 1));
}
// Determiant Method Test:
// Check determinant [5 3 -4  = 4
//                    2 0 -2
//                    2 5 -1]
#[test]
fn determinant_method_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let mut m1: Matrix<isize> = matrix(3, 3);
  m1.set(0, 0, 5); m1.set(0, 1, 3); m1.set(0, 2, -4);
  m1.set(1, 0, 2); m1.set(1, 1, 0); m1.set(1, 2, -2);
  m1.set(2, 0, 2); m1.set(2, 1, 5); m1.set(2, 2, -1);
  let m = m1.det();
  assert!(4 == m);
}
// Inverse Method Test - N/A for integer types
// Printing complex to terminal
#[test]
fn print_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  let mut m: Matrix<isize> = matrix(3, 3);
  m.set(0, 0, 1); m.set(0, 1, 2); m.set(0, 2, -3);
  m.set(1, 0, 6); m.set(1, 1, -9); m.set(1, 2, 8);
  m.set(2, 0, 3); m.set(2, 1, 7); m.set(2, 2, -9);
  println!("M is {}", m);
}
