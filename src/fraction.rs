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

#[derive(Clone)]
#[derive(Copy)]
// Data definition
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
