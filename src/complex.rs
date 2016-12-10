//! NAME
//!  complex.rs
//!
//! DESCRIPTION
//!  Module file that defines the complex class
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
// Derivations
#[derive(Clone)]
#[derive(Copy)]

// Data definition
pub struct Complex {
  real: f64,
  imag: f64,
}

const COMPLEX_TOL: f64 = 0.00001;

// Constructors
pub fn complex(r: f64, i: f64) -> Complex {
  Complex {real: r, imag: i}
}

// Methods
impl Complex {
  // Getters
  pub fn re(&self) -> f64 { self.real }
  pub fn im(&self) -> f64 { self.imag }

  // Setters
  pub fn set_re(&mut self, r: f64) { self.real = r; }
  pub fn set_im(&mut self, i: f64) { self.imag = i; }

  // Conjugate method
  pub fn conjugate(&self) -> Complex {
    Complex {real: self.re(), imag: -self.im()}
  }

  // Utility methods
  // Removes negative zero from complex number representation
  fn delnegzero(&self) -> Complex {
    if (-0.0 == self.re()) && (-0.0 == self.im()) {
      Complex {real: 0.0, imag: 0.0}
    } else if -0.0 == self.re() {
      Complex {real: 0.0, imag: self.im()}
    } else if -0.0 == self.im() {
      Complex {real: self.re(), imag: 0.0}
    } else {
      Complex {real: self.re(), imag: self.im()}
    }
  }
  // Checks if complex number is purely real
  fn isreal(&self) -> bool {
    COMPLEX_TOL > self.im().abs()
  }
}
  /*
  // Raising fraction to a power method
  pub fn pow(&self, exp: isize) -> Complex {
    
    if 0 > exp {
      // Negative exponents flip the fraction
      let exp_u32 = -exp as u32;
      let f = Complex {real: self.den().pow(exp_u32), imag: self.num().pow(exp_u32)};
      f.reduce()
    } else {
      let exp_u32 = exp as u32;
      let f = Complex {real: self.num().pow(exp_u32), imag: self.den().pow(exp_u32)};
      f.reduce()
    }
  }
}*/

// Arithmetic operations & operator overload
impl ::std::ops::Add for Complex {
  type Output = Complex;
  fn add(self, other: Complex) -> Complex {
    let c = Complex {real: self.re() + other.re(), imag: self.im() + other.im()};
    c.delnegzero()
  }
}
impl ::std::ops::Sub for Complex {
  type Output = Complex;
  fn sub(self, other: Complex) -> Complex {
    let c = Complex {real: self.re() - other.re(), imag: self.im() - other.im()};
    c.delnegzero()
  }
}
impl ::std::ops::Mul for Complex {
  type Output = Complex;
  fn mul(self, other:Complex) -> Complex {
    let c = Complex {real: self.re()*other.re() - self.im()*other.im(), 
                     imag: self.im()*other.re() + self.re()*other.im()};
    c.delnegzero()
  }
}
impl ::std::ops::Div for Complex {
  type Output = Complex;
  fn div(self, other:Complex) -> Complex {
    let mut num = self * other.conjugate();
    let mut den = other * other.conjugate();
    num = num.delnegzero();
    den = den.delnegzero();
    if 0.0 == den.re() {
      panic!("Attempted to divide by zero.");
    } else if !den.isreal() {
      panic!("Complex conjugate did not produce real number in denominator.");
    } else {
      let c = Complex {real: num.re() / den.re(), imag: num.im() / den.re()};
      c.delnegzero()
    }
  }
}
/*
// Unary operator overload
impl ::std::ops::Neg for Complex {
  type Output = Complex;
  fn neg(self) -> Complex {
    let f = Complex {real: -self.num(), imag: self.den()};
    f.reduce()
  }
}

// Comparison operator overloads
use ::std::cmp::Ordering;
impl ::std::cmp::PartialEq for Complex {
  fn eq(&self, other: &Complex) -> bool {
    self.num() * other.den() == self.den() * other.num()
  }
}
impl ::std::cmp::Eq for Complex {}
impl ::std::cmp::PartialOrd for Complex {
  fn partial_cmp(&self, other: &Complex) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
impl Ord for Complex {
  fn cmp(&self, other: &Complex) -> Ordering {
    if (0 > self.den()) ||(0 > other.den()) {
      // Need to add negative sign to num() and f.num() because 
      // negative in imag causes comparison to be flipped
      (-self.num() * other.den()).cmp(&(-other.num() * self.den()))
    } else {
      (self.num() * other.den()).cmp(&(other.num() * self.den()))
    }
  }
}

// Print formatting
impl ::std::fmt::Display for Complex {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    write!(f, "{}/{}", self.num(), self.den())
  }
}
*/
