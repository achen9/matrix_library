//! NAME
//!  matrix.rs
//!
//! DESCRIPTION
//!  Module file that defines the matrix class
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
extern crate rand;

use ::std::vec::Vec;
use fraction;
use complex;
use self::rand::{thread_rng, Rng};

// Data definition
#[derive(Clone)]
pub struct Matrix<T> {
  num_rows: usize,
  num_columns: usize,
  values: Vec<T>,
}

// Whole number trait
// This is used to create whole numbers for generic types. This enables setting the 
// elements in a newly constructed matrix to zero, and making named constructors such 
// as identity(), or ones().
pub trait WholeNum<T> {
  fn whole_num(val: isize) -> T;
}
impl WholeNum<isize> for isize {
  fn whole_num(val: isize) -> isize {
    val
  }
}
impl WholeNum<f64> for f64 {
  fn whole_num(val: isize) -> f64 {
    val as f64
  }
}
impl WholeNum<fraction::Fraction> for fraction::Fraction {
  fn whole_num(val: isize) -> fraction::Fraction {
    fraction::fraction(val, 1)
  }
}
impl WholeNum<complex::Complex> for complex::Complex {
  fn whole_num(val: isize) -> complex::Complex {
    complex::complex(val as f64, 0.0)
  }
}
impl WholeNum<bool> for bool {
  fn whole_num(val: isize) -> bool {
    if 0 != val {
      true
    } else {
      false
    }
  }
}

// Constructor
pub fn matrix<T: Copy + WholeNum<T>>(r: usize, c: usize) -> Matrix<T> {
  if (0 == r) || (0 == c) {
    panic!("Attempted to initialize a matrix with non-positive number of rows and/or columns.");
  }
  let mut m = Matrix {num_rows: r, num_columns: c, values: Vec::with_capacity(r*c)};
  let default: T = T::whole_num(0);
  for i in 0..(r*c) {
    m.values.push(default);
  }
  m
}
pub fn identity<T: Copy + WholeNum<T>>(n: usize) -> Matrix<T> {
  if 0 == n {
    panic!("Attempted to initizalize a ones matrix with non-positive number of rows and/or columns.");
  }
  let mut m = matrix(n, n);
  let one: T = T::whole_num(1);
  for i in 0..n {
      m.set(i, i, one);
  }
  m
}
pub fn ones<T: Copy + WholeNum<T>>(r: usize, c: usize) -> Matrix<T> {
  if (0 == r) && (0 == c) {
    panic!("Attempted to initizalize a ones matrix with non-positive number of rows & columns.");
  }
  let mut m = matrix(r, c);
  let one: T = T::whole_num(1);
  for i in 0..r {
    for j in 0..c {
      m.set(i, j, one);
    }
  }
  m
}
pub fn random<T: Copy + WholeNum<T>>(r: usize, c: usize) -> Matrix<T> {
   if (0 == r) && (0 == c) {
    panic!("Attempted to initizalize a ones matrix with non-positive number of rows & columns.");
  }
  let mut m = matrix(r, c);
  let mut rng = thread_rng();
  for i in 0..r {
    for j in 0..c {
      let random: u32 = rng.gen();
      m.set(i, j, T::whole_num((random % 20) as isize - 10));
    }
  }
  m
}

// Methods
impl<T: Copy + WholeNum<T>> Matrix<T> {
  // Getters
  pub fn rows(&self) -> usize { self.num_rows }
  pub fn columns(&self) -> usize { self.num_columns }
  pub fn get(&self, r: usize, c: usize) -> T {
    if !self.in_range(r, c) {
      panic!("Indices out of range in call to matrix::get().");
    }
    let val = self.values[r*self.columns() + c];
    val
  }

  // Setters
  pub fn set(&mut self, r: usize, c: usize, x: T) {
    if !self.in_range(r, c) {
      panic!("Indices out of range in call to matrix::set().");
    }
    self.values[r*self.num_columns + c] = x;
  }

  // Utility methods
  fn in_range(&self, r: usize, c: usize) -> bool {
    (r < self.rows()) && (c < self.columns())
  }
  pub fn transpose(&self) -> Matrix<T> {
    let mut m: Matrix<T> = matrix(self.columns(), self.rows());
    for i in 0..m.rows() {
      for j in 0..m.columns() {
        m.set(i, j, self.get(j, i));
      }
    }
    m
  }
  pub fn minor(&self, r: usize, c: usize) -> Matrix<T> {
    if !self.in_range(r, c) {
      panic!("Attempted to take minor with out of range indices.");
    }
    let mut m: Matrix<T> = matrix(self.rows() - 1, self.columns() - 1);
    let mut I = 0;
    let mut J;
    for i in 0..self.rows() {
      J = 0;
      for j in 0..self.columns() {
        if (i != r) && (j != c) {
          m.set(I, J, self.get(i, j));
          J += 1;
        }
      }
      if i != r {
        I += 1;
      }
    }
    m
  }
}

// Matrix utility methods
impl<T> Matrix<T> 
  where T: Copy + 
           ::std::ops::Add<Output=T> + 
           ::std::ops::Sub<Output=T> + 
           ::std::ops::Mul<Output=T> + 
           ::std::ops::Div<Output=T> +
           ::std::ops::Neg<Output=T> +
           ::std::cmp::PartialEq +
           WholeNum<T> {
  pub fn scale(&self, s: T) -> Matrix<T> {
    let mut m: Matrix<T> = matrix(self.rows(), self.columns());
    for i in 0..self.rows() {
      for j in 0..self.columns() {
        m.set(i, j, s * self.get(i, j));
      }
    }
    m
  }
  pub fn det(&self) -> T {
    if self.rows() != self.columns() {
      panic!("Attempted to find determinant of non-square matrix.");
    } else if (1 == self.rows()) && (1 == self.columns()) {
      self.get(0, 0)
    } else {
      let mut sum = self.get(0, 0) * self.minor(0, 0).det(); 
      let mut sign = -1;
      for i in 1..self.columns() {
        if 0 < sign {
          sum = sum + self.get(0, i) * self.minor(0, i).det();
        } else {
          sum = sum - self.get(0, i) * self.minor(0, i).det();
        }
        sign *= -1;
      }
      sum
    }
  }
  pub fn inverse(&self) -> Matrix<T> {
    if self.rows() != self.columns() {
      panic!("Attempted to invert non-invertible matrix.");
    }
    let d: T = self.det();
    if d == T::whole_num(0) {
      panic!("Determinant is zero to working precision.");
    }
    let mut m: Matrix<T> = matrix(self.rows(), self.columns());
    for i in 0..self.rows() {
      for j in 0..self.columns() {
        let mut sign;
        if 0 == (i + j) % 2 {
          sign = 1;
        } else {
          sign = -1;
        }
        if 0 < sign {
          m.set(i, j, self.minor(j, i).det() / d);
        } else {
          m.set(i, j, -self.minor(j, i).det() / d);
        }
      }
    }
    m
  }
}

// Arithmetic operations & operator overload
impl<T: Copy + ::std::ops::Add<Output=T> + WholeNum<T>> ::std::ops::Add for Matrix<T> {
  type Output = Matrix<T>;
  fn add(self, other: Matrix<T>) -> Matrix<T> {
    if (self.rows() != other.rows()) || (self.columns() != other.columns()) {
      panic!("Attempted to add matrices with incompatible sizes.");
    }
    let mut m: Matrix<T> = matrix(self.rows(), self.columns());
    for i in 0..self.rows() {
      for j in 0..self.columns() {
        m.set(i, j, self.get(i, j) + other.get(i, j));
      }
    }
    m
  }
}
impl<T: Copy + ::std::ops::Sub<Output=T> + WholeNum<T>> ::std::ops::Sub for Matrix<T> {
  type Output = Matrix<T>;
  fn sub(self, other: Matrix<T>) -> Matrix<T> {
    if (self.rows() != other.rows()) || (self.columns() != other.columns()) {
      panic!("Attempted to subtract matrices with incompatible sizes.");
    }
    let mut m: Matrix<T> = matrix(self.rows(), self.columns());
    for i in 0..self.rows() {
      for j in 0..self.columns() {
        m.set(i, j, self.get(i, j) - other.get(i, j));
      }
    }
    m
  }
}
impl<T: Copy + ::std::ops::Mul<Output=T> + ::std::ops::Add<Output=T> + WholeNum<T>> ::std::ops::Mul for Matrix<T> {
  type Output = Matrix<T>;
  fn mul(self, other:Matrix<T>) -> Matrix<T> {
    if self.columns() != other.rows() {
      panic!("Attempted to multiply matrices with incompatible sizes.");
    }
    let mut m: Matrix<T> = matrix(self.rows(), other.columns());
    for i in 0..self.rows() {
      for j in 0..other.columns() {
        for k in 0..self.columns() {
          if 0 == k {
            m.set(i, j, self.get(i, k) * other.get(k, j));
          } else {
            let tmp = m.get(i, j) + self.get(i, k) * other.get(k, j);
            m.set(i, j, tmp);
          }
        }
      }
    }
    m
  }
}

// Comparison operator overloads
impl<T: WholeNum<T>> ::std::cmp::PartialEq for Matrix<T> 
  where T: Copy + 
        ::std::cmp::PartialEq {
  fn eq(&self, other: &Matrix<T>) -> bool {
    for i in 0..self.rows() {
      for j in 0..self.columns() {
        if self.get(i, j) != other.get(i, j) {
          return false
        }
      }
    }
    return true
  }
}

// Print formatting
impl<T: Copy + ::std::fmt::Display + WholeNum<T>> ::std::fmt::Display for Matrix<T> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    write!(f, "\n");
    for i in 0..self.rows() {
      write!(f, "[");
      for j in 0..self.columns() {
        if self.columns() == (j + 1) {
          write!(f, "{}]\n", self.get(i, j));
        } else {
          write!(f, "{}\t", self.get(i, j));
        }
      }
    }
    write!(f, "\n")
  }
}
