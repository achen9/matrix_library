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
use ::std::vec::Vec;

// Data definition
#[derive(Clone)]
pub struct Matrix<T> {
  num_rows: usize,
  num_columns: usize,
  values: Vec<T>,
}

// Constructor
pub fn matrix<T>(r: usize, c: usize) -> Matrix<T> {
  if (0 >= r) || (0 >= c) {
    panic!("Attempted to initialize a matrix with non-positive number of rows and/or columns.");
  }
  let mut m = Matrix {num_rows: r, num_columns: c, values: Vec::with_capacity(r*c)};
  // Due to no "zero" equivalent for all types (fraction and complex class specifically),
  // need to allocate space in vector for number of elements in matrix. Then need to use
  // unsafe code to explicitly set the vector length to equal the previously allocated space.
  unsafe {
    m.values.set_len(r*c);
  }
  m
}

// Methods
impl <T: Copy> Matrix<T> {
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
}
