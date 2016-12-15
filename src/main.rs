extern crate matrix_lib;

fn main() {
  use matrix_lib::matrix::{Matrix, matrix, identity, random};
  use matrix_lib::fraction::Fraction;
  use matrix_lib::complex::Complex;
  let m_f: Matrix<Fraction> = random(3,3);
  let m_c: Matrix<Complex> = random(3,3);
  let m_b: Matrix<bool> = identity(3);
  println!("Matrix m_f is: {}Matrix m_c is: {}Matrix m_b is: {}", m_f, m_c, m_b);
}
