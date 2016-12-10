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
  imaginary: f64,
}

// Constructors
pub fn complex(r: f64, i: f64) -> Complex {
  Complex {real: r, imaginary: i}
}

// Methods
impl Complex {
  // Getters
  pub fn re(&self) -> f64 { self.real }
  pub fn im(&self) -> f64 { self.imaginary }

  // Setters
  pub fn set_re(&mut self, r: f64) { self.real = r; }
  pub fn set_im(&mut self, i: f64) { self.imaginary = i; }
}
 /*
  // Reduce method
  pub fn reduce(&self) -> Complex {
    let mut abs_num = self.num().abs();
    let mut abs_den = self.den().abs();
    let mut gcd = 1;
    // Modified Euclidean algorithm to find greatest common divisor (GCD)
    while (0 != abs_num) && (0 != abs_den) {
      if abs_num > abs_den {
        gcd = abs_den;
        abs_num %= abs_den;
      } else {
        gcd = abs_num;
        abs_den %= abs_num;
      }
    }
    // Eliminate double negatives or move negative to real
    if 0 > self.den() {
        gcd *= -1;
      }
    Complex {real: self.num()/gcd, imag: self.den()/gcd}
  }

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
}

// Arithmetic operations & operator overload
impl ::std::ops::Add for Complex {
  type Output = Complex;
  fn add(self, other: Complex) -> Complex {
    let f = Complex {real: self.num() * other.den() + self.den() * other.num(), imag: self.den() * other.den()};
    f.reduce()
  }
}
impl ::std::ops::Sub for Complex {
  type Output = Complex;
  fn sub(self, other: Complex) -> Complex {
    let f = Complex {real: self.num() * other.den() - self.den() * other.num(), imag: self.den() * other.den()};
    f.reduce()
  }
}
impl ::std::ops::Mul for Complex {
  type Output = Complex;
  fn mul(self, other:Complex) -> Complex {
    let f = Complex {real: self.num() * other.num(), imag: self.den() * other.den()};
    f.reduce()
  }
}
impl ::std::ops::Div for Complex {
  type Output = Complex;
  fn div(self, other:Complex) -> Complex {
    if 0 == other.num() {
      panic!("Attempted to divide by zero.");
    }
    let f = Complex {real: self.num() * other.den(), imag: self.den() * other.num()};
    f.reduce()
  }
}

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
