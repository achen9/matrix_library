//! NAME
//!  fraction.rs
//!
//! DESCRIPTION
//!  Module file that defines the fraction class
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

// Data Definition
#[derive(Clone, Copy)]
pub struct Fraction {
  numerator: isize,
  denominator: isize,
}

// Constructors
pub fn fraction(n: isize, d: isize) -> Fraction {
  if 0 == d {
    panic!("Attempted to set denominator to 0");
  } 
  Fraction {numerator: n, denominator: d}
}

// Methods
impl Fraction {
  // Getters
  pub fn num(&self) -> isize { self.numerator }
  pub fn den(&self) -> isize { self.denominator }

  // Setters
  pub fn set_num(&mut self, n: isize) { self.numerator = n; }
  pub fn set_den(&mut self, d: isize) {
    if 0 == d {
      panic!("Attempted to set denominator to 0");
    }
    self.denominator = d;
  }

  // Reduce method
  pub fn reduce(&self) -> Fraction {
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
    // Eliminate double negatives or move negative to numerator
    if 0 > self.den() {
        gcd *= -1;
      }
    Fraction {numerator: self.num()/gcd, denominator: self.den()/gcd}
  }

  // Raising fraction to a power method
  pub fn pow(&self, exp: isize) -> Fraction {
    
    if 0 > exp {
      // Negative exponents flip the fraction
      let exp_u32 = -exp as u32;
      let f = Fraction {numerator: self.den().pow(exp_u32), denominator: self.num().pow(exp_u32)};
      f.reduce()
    } else {
      let exp_u32 = exp as u32;
      let f = Fraction {numerator: self.num().pow(exp_u32), denominator: self.den().pow(exp_u32)};
      f.reduce()
    }
  }
}

// Arithmetic operations & operator overload
impl ::std::ops::Add for Fraction {
  type Output = Fraction;
  fn add(self, other: Fraction) -> Fraction {
    let f = Fraction {numerator: self.num() * other.den() + self.den() * other.num(), denominator: self.den() * other.den()};
    f.reduce()
  }
}
impl ::std::ops::Sub for Fraction {
  type Output = Fraction;
  fn sub(self, other: Fraction) -> Fraction {
    let f = Fraction {numerator: self.num() * other.den() - self.den() * other.num(), denominator: self.den() * other.den()};
    f.reduce()
  }
}
impl ::std::ops::Mul for Fraction {
  type Output = Fraction;
  fn mul(self, other:Fraction) -> Fraction {
    let f = Fraction {numerator: self.num() * other.num(), denominator: self.den() * other.den()};
    f.reduce()
  }
}
impl ::std::ops::Div for Fraction {
  type Output = Fraction;
  fn div(self, other:Fraction) -> Fraction {
    if 0 == other.num() {
      panic!("Attempted to divide by zero.");
    }
    let f = Fraction {numerator: self.num() * other.den(), denominator: self.den() * other.num()};
    f.reduce()
  }
}

// Unary operator overload
impl ::std::ops::Neg for Fraction {
  type Output = Fraction;
  fn neg(self) -> Fraction {
    let f = Fraction {numerator: -self.num(), denominator: self.den()};
    f.reduce()
  }
}

// Comparison operator overloads
use ::std::cmp::Ordering;
impl ::std::cmp::PartialEq for Fraction {
  fn eq(&self, other: &Fraction) -> bool {
    self.num() * other.den() == self.den() * other.num()
  }
}
impl ::std::cmp::Eq for Fraction {}
impl ::std::cmp::PartialOrd for Fraction {
  fn partial_cmp(&self, other: &Fraction) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
impl Ord for Fraction {
  fn cmp(&self, other: &Fraction) -> Ordering {
    if (0 > self.den()) ||(0 > other.den()) {
      // Need to add negative sign to num() and f.num() because 
      // negative in denominator causes comparison to be flipped
      (-self.num() * other.den()).cmp(&(-other.num() * self.den()))
    } else {
      (self.num() * other.den()).cmp(&(other.num() * self.den()))
    }
  }
}

// Print formatting
impl ::std::fmt::Display for Fraction {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    write!(f, "{}/{}", self.num(), self.den())
  }
}
