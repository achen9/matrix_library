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

// Constructor
pub fn dft(n: usize) -> DFT {
  if 0 == n {
    panic!("Attempted to initialize a DFT vector with zero length.");
  }
  DFT {num_pts: n, cache: HashMap::new()}
}

// Methods
impl DFT {
  // Getters
  pub fn npts(&self) -> usize { self.num_pts }
}
