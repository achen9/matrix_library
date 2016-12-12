//! NAME
//!  matrix_unit_tests_fraction.rs
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
  let mut m: Matrix<Fraction> = matrix(2, 2);
  use matrix_lib::fraction::{Fraction, fraction};
  let f1 = fraction(1, 3); let f2 = fraction(-2, 3);
  let f3 = fraction(3, 2); let f4 = fraction(-9, 5);
  m.set(0, 0, f1); m.set(0, 1, f2);
  m.set(1, 0, f3); m.set(1, 1, f4);
  assert!(2 == m.rows());
  assert!(2 == m.columns());
  assert!(f1 == m.get(0, 0));
  assert!(f2 == m.get(0, 1));
  assert!(f3 == m.get(1, 0));
  assert!(f4 == m.get(1, 1));
}
// Copy Constructor Test: Test if 2x2 matrix can be copied
#[test]
fn copy_constructor_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::fraction::{Fraction, fraction};
  let mut m1: Matrix<Fraction> = matrix(2, 2);
  let f1 = fraction(1, 3); let f2 = fraction(-2, 3);
  let f3 = fraction(3, 2); let f4 = fraction(-9, 5);
  m1.set(0, 0, f1); m1.set(0, 1, f2);
  m1.set(1, 0, f3); m1.set(1, 1, f4);
  let mut m: Matrix<Fraction> = m1.clone();
  assert!(f1 == m.get(0, 0));
  assert!(f2 == m.get(0, 1));
  assert!(f3 == m.get(1, 0));
  assert!(f4 == m.get(1, 1));
  let f5 = fraction(1, 2);
  m.set(1, 0, f5);
  assert!(f5 == m.get(1, 0));
  assert!(f3 == m1.get(1, 0));
}
// Arithmetic Operation Overload Test 1:
// Check [1/2 -2/3  + [1/3 -2/5   =  [5/6 -16/15
//        3/5 -9/6]    3/9 -9/3]      14/15 -9/2]
#[test]
fn addition_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::fraction::{Fraction, fraction};
  let mut m1: Matrix<Fraction> = matrix(2, 2);
  let m11 = fraction(1, 2); let m12 = fraction(-2, 3);
  let m13 = fraction(3, 5); let m14 = fraction(-9, 6);
  m1.set(0, 0, m11); m1.set(0, 1, m12);
  m1.set(1, 0, m13); m1.set(1, 1, m14);
  let mut m2: Matrix<Fraction> = matrix(2, 2);
  let m21 = fraction(1, 3); let m22 = fraction(-2, 5);
  let m23 = fraction(3, 9); let m24 = fraction(-9, 3);
  m2.set(0, 0, m21); m2.set(0, 1, m22);
  m2.set(1, 0, m23); m2.set(1, 1, m24);
  let m = m1 + m2;
  let ma = fraction(5, 6); let mb = fraction(-16, 15);
  let mc = fraction(14, 15); let md = fraction(-9, 2);
  assert!(ma == m.get(0, 0));
  assert!(mb == m.get(0, 1));
  assert!(mc == m.get(1, 0));
  assert!(md == m.get(1, 1));
}
// Arithmetic Operation Overload Test 2: Check 2x2 matrix + 2x3 matrix => panics
#[test]
#[should_panic]
fn addition_error_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::fraction::{Fraction, fraction};
  let mut m1: Matrix<Fraction> = matrix(2, 2);
  let m11 = fraction(1, 2); let m12 = fraction(-2, 3);
  let m13 = fraction(3, 5); let m14 = fraction(-9, 6);
  m1.set(0, 0, m11); m1.set(0, 1, m12);
  m1.set(1, 0, m13); m1.set(1, 1, m14);
  let mut m2: Matrix<Fraction> = matrix(2, 3);
  let m21 = fraction(1, 3); let m22 = fraction(-2, 5); let m23 = fraction(2, 3);
  let m24 = fraction(3, 9); let m25 = fraction(-9, 3); let m26 = fraction(1, 3);
  m2.set(0, 0, m21); m2.set(0, 1, m22); m2.set(0, 2, m23);
  m2.set(1, 0, m24); m2.set(1, 1, m25); m2.set(1, 2, m26);
  let m = m1 + m2;
  assert!(true); // Something went wrong if this assertion passes
}
// Arithmetic Operation Overload Test 3: 
// Check [1/2 -2/3  - [1/3 -2/5   =  [1/6 -4/15
//        3/5 -9/6]    3/9 -9/3]      4/15 3/2]
#[test]
fn subtraction_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::fraction::{Fraction, fraction};
  let mut m1: Matrix<Fraction> = matrix(2, 2);
  let m11 = fraction(1, 2); let m12 = fraction(-2, 3);
  let m13 = fraction(3, 5); let m14 = fraction(-9, 6);
  m1.set(0, 0, m11); m1.set(0, 1, m12);
  m1.set(1, 0, m13); m1.set(1, 1, m14);
  let mut m2: Matrix<Fraction> = matrix(2, 2);
  let m21 = fraction(1, 3); let m22 = fraction(-2, 5);
  let m23 = fraction(3, 9); let m24 = fraction(-9, 3);
  m2.set(0, 0, m21); m2.set(0, 1, m22);
  m2.set(1, 0, m23); m2.set(1, 1, m24);
  let m = m1 - m2;
  let ma = fraction(1, 6); let mb = fraction(-4, 15);
  let mc = fraction(4, 15); let md = fraction(3, 2);
  assert!(ma == m.get(0, 0));
  assert!(mb == m.get(0, 1));
  assert!(mc == m.get(1, 0));
  assert!(md == m.get(1, 1));
}
// Arithmetic Operation Overload Test 4: Check 2x2 matrix - 2x3 matrix => panics
#[test]
#[should_panic]
fn subtraction_error_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::fraction::{Fraction, fraction};
  let mut m1: Matrix<Fraction> = matrix(2, 2);
  let m11 = fraction(1, 2); let m12 = fraction(-2, 3);
  let m13 = fraction(3, 5); let m14 = fraction(-9, 6);
  m1.set(0, 0, m11); m1.set(0, 1, m12);
  m1.set(1, 0, m13); m1.set(1, 1, m14);
  let mut m2: Matrix<Fraction> = matrix(2, 3);
  let m21 = fraction(1, 3); let m22 = fraction(-2, 5); let m23 = fraction(2, 3);
  let m24 = fraction(3, 9); let m25 = fraction(-9, 3); let m26 = fraction(1, 3);
  m2.set(0, 0, m21); m2.set(0, 1, m22); m2.set(0, 2, m23);
  m2.set(1, 0, m24); m2.set(1, 1, m25); m2.set(1, 2, m26);
  let m = m1 - m2;
  assert!(true); // Something went wrong if this assertion passes
}
// Arithmetic Operation Overload Test 5:
// Check [1/2  - [1/3 -2/5]   =  [1/6 -1/5
//        3/5]                    1/5 -6/25]
#[test]
fn multiplication_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::fraction::{Fraction, fraction};
  let mut m1: Matrix<Fraction> = matrix(2, 1);
  let m11 = fraction(1, 2); let m12 = fraction(3, 5);
  m1.set(0, 0, m11); m1.set(1, 0, m12);
  let mut m2: Matrix<Fraction> = matrix(1, 2);
  let m21 = fraction(1, 3); let m22 = fraction(-2, 5);
  m2.set(0, 0, m21); m2.set(0, 1, m22);
  let m = m1 * m2;
  let ma = fraction(1, 6); let mb = fraction(-1, 5);
  let mc = fraction(1, 5); let md = fraction(-6, 25);
  assert!(ma == m.get(0, 0));
  assert!(mb == m.get(0, 1));
  assert!(mc == m.get(1, 0));
  assert!(md == m.get(1, 1));
}
// Arithmetic Operation Overload Test 6: Check 2x1 matrix * 2x1 matrix => panics
#[test]
#[should_panic]
fn multiplication_error_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::fraction::{Fraction, fraction};
  let mut m1: Matrix<Fraction> = matrix(2, 1);
  let m11 = fraction(1, 2); let m12 = fraction(-2, 3);
  m1.set(0, 0, m11); m1.set(1, 0, m12);
  let mut m2: Matrix<Fraction> = matrix(2, 1);
  let m21 = fraction(1, 3); let m22 = fraction(-2, 5);
  m2.set(0, 0, m21); m2.set(1, 0, m22);
  let m = m1 * m2;
  assert!(true); // Something went wrong if this assertion passes
}
// Scale Method Test
// Check [1/2 -2/3  * 2/5 = [1/5 -4/15
//        3/5 -9/6]          6/25 -3/5]
#[test]
fn scale_method_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::fraction::{Fraction, fraction};
  let mut m1: Matrix<Fraction> = matrix(2, 2);
  let m11 = fraction(1, 2); let m12 = fraction(-2, 3);
  let m13 = fraction(3, 5); let m14 = fraction(-9, 6);
  m1.set(0, 0, m11); m1.set(0, 1, m12);
  m1.set(1, 0, m13); m1.set(1, 1, m14);
  let s = fraction(2, 5);
  let m = m1.scale(s);
  let ma = fraction(1, 5); let mb = fraction(-4, 15);
  let mc = fraction(6, 25); let md = fraction(-3, 5);
  assert!(ma == m.get(0, 0));
  assert!(mb == m.get(0, 1));
  assert!(mc == m.get(1, 0));
  assert!(md == m.get(1, 1));
}
// Transpose Method Test
// Check [1/2]T = [1/2 3/5]
//       [3/5]     
#[test]
fn transpose_method_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::fraction::{Fraction, fraction};
  let mut m1: Matrix<Fraction> = matrix(2, 1);
  let m11 = fraction(1, 2); let m12 = fraction(3, 5);
  m1.set(0, 0, m11); m1.set(1, 0, m12);
  let m = m1.transpose();
  let ma = fraction(1, 2); let mb = fraction(3, 5);
  assert!(1 == m.rows());
  assert!(2 == m.columns());
  assert!(ma == m.get(0, 0));
  assert!(mb == m.get(0, 1));
}
// Minor Method Test
// Check [1/2 1/5  -2/3  => (1, 1) minor => [1/2 -2/3
//        2/3 1/4  -9/4                      3/5 -9/6]
//        3/5 -8/9 -9/6]          
#[test]
fn minor_method_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::fraction::{Fraction, fraction};
  let mut m1: Matrix<Fraction> = matrix(3, 3);
  let m11 = fraction(1, 2); let m12 = fraction(1, 5); let m13 = fraction(-2, 3);
  let m14 = fraction(2, 3); let m15 = fraction(1, 4); let m16 = fraction(-9, 4);
  let m17 = fraction(3, 5); let m18 = fraction(-8, 9); let m19 = fraction(-9, 6);
  m1.set(0, 0, m11); m1.set(0, 1, m12); m1.set(0, 2, m13);
  m1.set(1, 0, m14); m1.set(1, 1, m15); m1.set(1, 2, m16);
  m1.set(2, 0, m17); m1.set(2, 1, m18); m1.set(2, 2, m19);
  let m = m1.minor(1, 1);
  assert!(m11 == m.get(0, 0));
  assert!(m13 == m.get(0, 1));
  assert!(m17 == m.get(1, 0));
  assert!(m19 == m.get(1, 1));
}
// Determinant Method Test:
// Check determinant [5/1 3/1 -4/1  = 4/1
//                    2/1 0/1 -2/1                                     
//                    2/1 5/1 -1/1]  
#[test]
fn determinant_method_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::fraction::{Fraction, fraction};
  let mut m1: Matrix<Fraction> = matrix(3, 3);
  let m11 = fraction(5, 1); let m12 = fraction(3, 1); let m13 = fraction(-4, 1);
  let m14 = fraction(2, 1); let m15 = fraction(0, 1); let m16 = fraction(-2, 1);
  let m17 = fraction(2, 1); let m18 = fraction(5, 1); let m19 = fraction(-1, 1);
  m1.set(0, 0, m11); m1.set(0, 1, m12); m1.set(0, 2, m13);
  m1.set(1, 0, m14); m1.set(1, 1, m15); m1.set(1, 2, m16);
  m1.set(2, 0, m17); m1.set(2, 1, m18); m1.set(2, 2, m19);
  let m = m1.det();
  let f = fraction(4, 1);
  assert!(f == m);
}
// Determinant Method Test:
// Check inverse [5/1 3/1 -4/1  = [ 5/2 -17/4 -3/2
//                2/1 0/1 -2/1     -1/2  3/4   1/2                                
//                2/1 5/1 -1/1]     5/2 -19/4 -3/2]
#[test]
fn inverse_method_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::fraction::{Fraction, fraction};
  let mut m1: Matrix<Fraction> = matrix(3, 3);
  let m11 = fraction(5, 1); let m12 = fraction(3, 1); let m13 = fraction(-4, 1);
  let m14 = fraction(2, 1); let m15 = fraction(0, 1); let m16 = fraction(-2, 1);
  let m17 = fraction(2, 1); let m18 = fraction(5, 1); let m19 = fraction(-1, 1);
  m1.set(0, 0, m11); m1.set(0, 1, m12); m1.set(0, 2, m13);
  m1.set(1, 0, m14); m1.set(1, 1, m15); m1.set(1, 2, m16);
  m1.set(2, 0, m17); m1.set(2, 1, m18); m1.set(2, 2, m19);
  let mut f: Matrix<Fraction> = matrix(3, 3);
  let f1 = fraction(5, 2); let f2 = fraction(-17, 4); let f3 = fraction(-3, 2);
  let f4 = fraction(-1, 2); let f5 = fraction(3, 4); let f6 = fraction(1, 2);
  let f7 = fraction(5, 2); let f8 = fraction(-19, 4); let f9 = fraction(-3, 2);
  f.set(0, 0, f1); f.set(0, 1, f2); f.set(0, 2, f3);
  f.set(1, 0, f4); f.set(1, 1, f5); f.set(1, 2, f6);
  f.set(2, 0, f7); f.set(2, 1, f8); f.set(2, 2, f9);
  let m = m1.inverse();
  assert!(f == m);
}
// Printing Matrix to Terminal
#[test]
fn print_test() {
  use matrix_lib::matrix::{Matrix, matrix};
  use matrix_lib::fraction::{Fraction, fraction};
  let mut m: Matrix<Fraction> = matrix(3, 3);
  let m1 = fraction(1, 2); let m2 = fraction(1, 5); let m3 = fraction(-2, 3);
  let m4 = fraction(2, 3); let m5 = fraction(1, 4); let m6 = fraction(-9, 4);
  let m7 = fraction(3, 5); let m8 = fraction(-8, 9); let m9 = fraction(-9, 6);
  m.set(0, 0, m1); m.set(0, 1, m2); m.set(0, 2, m3);
  m.set(1, 0, m4); m.set(1, 1, m5); m.set(1, 2, m6);
  m.set(2, 0, m7); m.set(2, 1, m8); m.set(2, 2, m9);
  println!("M: is {}", m);
}
