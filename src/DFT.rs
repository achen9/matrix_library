//! NAME
//!  DFT.rs
//!
//! DESCRIPTION
//!  Module file that defines the DFT (Discrete Fourier Transform) class
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
use ::std::collections::HashMap;
use complex;
use matrix;

// Data definition
#[derive(Clone)]
pub struct DFT {
  num_pts: usize,
  cache: HashMap<String, matrix::Matrix<complex::Complex>>,
}

const PI: f64 = 3.14159;

// Constructor
pub fn dft(n: usize) -> DFT {
  if 0 == n {
    panic!("Attempted to initialize a DFT vector with zero length.");
  }
  DFT {num_pts: n, cache: HashMap::new()}
}

 // Utility Function
 fn conjugate_transpose(m: matrix::Matrix<complex::Complex>) -> matrix::Matrix<complex::Complex> {
  let t: matrix::Matrix<complex::Complex> = m.transpose();
  let mut ct: matrix::Matrix<complex::Complex> = matrix::matrix(t.rows(), t.columns());
  for i in 0..ct.rows() {
    for j in 0..ct.columns() {
      ct.set(i, j, t.get(i, j).conjugate());
    }
  }
  ct
}

// Methods
impl DFT {
  // Getters
  pub fn npts(&self) -> usize { self.num_pts }

  // Discrete Fourier Transform Methods
  pub fn transform_matrix(&mut self) -> matrix::Matrix<complex::Complex> {
    if self.cache.contains_key(&"Transform Matrix".to_string()) {
      match self.cache.get(&"Transform Matrix".to_string()) {
        Some(matrix) => matrix.clone(),
        None => panic!("No transform matrix found in DFT."),
      }
    } else {
      let exponent = complex::complex(0.0, -2.0*PI/(self.npts() as f64));
      let w = exponent.exp();
      let mut m: matrix::Matrix<complex::Complex> = matrix::matrix(self.npts(), self.npts());
      for i in 0..m.rows() {
        for j in 0..m.columns() {
          m.set(i, j, w.pow((i * j) as i32));
        }
      }
      self.cache.insert("Transform Matrix".to_string(), m.clone());
      m.clone()
    }
  }
  pub fn unitary_matrix(&mut self) -> matrix::Matrix<complex::Complex> {
    if self.cache.contains_key(&"Unitary Matrix".to_string()) {
      match self.cache.get(&"Unitary Matrix".to_string()) {
        Some(matrix) => matrix.clone(),
        None => panic!("No unitary matrix found in DFT."),
      }
    } else {
      let m: matrix::Matrix<complex::Complex> = self.transform_matrix();
      let s = complex::complex(1.0 / (self.npts() as f64).sqrt(), 0.0);
      let u: matrix::Matrix<complex::Complex> = m.scale(s);
      self.cache.insert("Unitary Matrix".to_string(), u.clone());
      u.clone()
    }
  }
  pub fn inverse_matrix(&mut self) -> matrix::Matrix<complex::Complex> {
    if self.cache.contains_key(&"Inverse Matrix".to_string()) {
       match self.cache.get(&"Inverse Matrix".to_string()) {
        Some(matrix) => matrix.clone(),
        None => panic!("No inverse matrix found in DFT."),
      }
    } else {
      let m: matrix::Matrix<complex::Complex> = conjugate_transpose(self.unitary_matrix());
      m.clone()
    }
  }
  pub fn dft_tfm(&mut self, vector: matrix::Matrix<complex::Complex>) -> matrix::Matrix<complex::Complex> {
    if (self.npts() != vector.rows()) || (1 != vector.columns()) {
      panic!("Incompatible input vector provided.");
    }
    self.unitary_matrix() * vector
  }
  pub fn dft_inv(&mut self, vector: matrix::Matrix<complex::Complex>) -> matrix::Matrix<complex::Complex> {
    if (self.npts() != vector.rows()) || (1 != vector.columns()) {
      panic!("Incompatible input vector provided.");
    }
    self.inverse_matrix() * vector
  }
}
