# Matrix Library v0.1.0 Documentation
This repository contains a matrix library implementation written in Rust. A simple set of matrix operations
such as arithmetic operations, determinants, inverse, and discrete Fourier transforms are provided.

Author: Alex Chen, alexac9@uw.edu
## Contents
1\.    Getting Started  
1.1.   Installing the Rust Compiler and Cargo Package Manager  
1.2.   Cloning the Library from the Repository   
1.3.   Using the Library  
1.4.   Running Unit Tests  
2\.    Library Content Documentation  
2.1.   Fraction Class  
2.1.1. Fraction Class Methods  
2.1.2. Fraction Class Arithmetic Operator Overloads  
2.1.3. Fraction Class Comparison Operator Overloads  
2.1.4. Printing Fractions to Stdout  
2.2.   Complex Class  
2.2.1. Complex Class Methods  
2.2.2. Complex Class Arithmetic Operator Overloads  
2.2.3. Complex Class Comparison Operator Overloads  
2.2.4. Printing Complex Numbers to Stdout  
2.3.   Matrix Class Generic  
2.3.1. Matrix Class Methods  
2.3.2. Matrix Class Arithmetic Operator Overloads  
2.3.3. Matrix Class Comparison Operator Overloads  
2.3.4. Printing Matrices to Stdout  
2.4.   Discrete Fourier Transform (DFT) Class  
2.4.1. DFT Class Methods  
3\.    Further Improvements  
4\.    Test Notes  

## 1. Getting Started
* Git is required to clone the matrix library respository.
* The Rust compiler and Cargo package manager are required to compile the matrix library. See section 1.1. 
for steps on installing the Rust compiler and Cargo package manager.

### 1.1. Installing the Rust Compiler and Cargo Package Manager
1. Download the Rust compiler and Cargo, the Rust package manager, for the latest stable release from
https://www.rust-lang.org/en-US/ for your operating system. Be sure to install Cargo as it will be used to 
compile the code.
2. Once installation has finished, open a terminal or command prompt and type "rustc --V" to make sure the 
Rust compiler was installed properly. You should see a version number (e.g. 1.13.0) of rustc listed.
3. Type "cargo -V" to make sure Cargo was installed properly. You should see a version number 
(e.g. 0.13.0-nightly) of Cargo listed.

### 1.2. Cloning the Library from the Repository
1. Make sure there is at least 10MB of hard drive space available to store the source files from the repository.
2. Check git has been installed on your system. Open a terminal or command prompt and type "git --version" to
to check if git has been installed. You should see a version number (e.g. 2.9.2). If git is not installed
on your system, go to https://git-scm.com/ to get the latest version of git for your system.
3. Open a terminal or command prompt and type "git clone https://github.com/achen9/matrix_library.git" to 
clone the source code.

### 1.3. Using the Library
Add "extern crate matrix_lib" to your source code to include the matrix library and its data types and methods.
Open a terminal or command prompt and navigate to the folder with the matrix library source files. Type
"cargo build" to compile the source code.

### 1.4. Running Unit Tests
Open a terminal or command prompt and navigate to the folder with the matrix library source files. Type:
* "cargo test" to run the built in unit tests. The source files will also be compiled if they have not 
been previously compiled, or the compiled source code is out of date.
* "cargo test -- --nocapture" to see the stdout for all unit tests. By default, unit tests will only 
print stdout to the terminal or command prompt if they fail. This option turns on stdout printing for 
all unit tests.
* "cargo run" to run the included example main.rs program which performs a benchmark test of the 
methods included in the matrix library.

## 2. Library Content Documentation
The following sections document the data types, methods, and design decisions contained in the 
matrix library.

### 2.1. Fraction Class
The "fraction" class is a data type for representing real numbers in rational form.

Note: Rust does not have a "class" keyword. Instead, classes can be simulated by using a struct
for storing data and using method syntax to allow function calls on the struct using the dot "." 
notation (i.e. struct.method()). Access to the struct fields must be done using the getter and 
setter methods. This is to prevent the possibility of  making inadvertent or invalid changes (e.g. 
setting the denominator to 0) to the fraction.

Data is stored in the following struct:
```rust
pub struct Fraction {
  numerator: isize,
  denominator: isize,
}
```
#### 2.1.1. Fraction Class Methods
In the following examples, be sure to add "extern crate matrix_lib" to the source code.

The **fraction constructor** creates an instance of a fraction. If a 0 is specified for the denominator,
the constructor will panic with an error message. Note, it is possible to have negative numbers in 
the denominator. This was included to simplify constructor code. Methods which operate on fractions
take this into account and will move the negative sign to the numerator when reducing the fraction.
```rust
pub fn fraction(n: isize, d: isize) -> Fraction
```
**Example:**
```rust
use matrix_lib::fraction::{Fraction, fraction};
let f:Fraction = fraction(2,3); // create fraction 2/3
```
The **getters** allow access to either the numerator or denominator.
```rust
pub fn num(&self) -> isize
pub fn den(&self) -> isize
```
**Example:**
```rust
use matrix_lib::fraction::{Fraction, fraction};
let f: Fraction = fraction(2,3);
let n: isize = f.num();
let d: isize = f.den();
```
The **setters** allow modification of the numerator or denominator. The set_den() method will panic if 
a 0 is input as the argument.
```rust
pub fn set_num(&mut self, n: isize)
pub fn set_den(&mut self, d: isize)
```
**Example:**
```rust
use matrix_lib::fraction::{Fraction, fraction};
let f: Fraction = fraction(2,3);
f.set_num(3); // numerator is now 3
f.set_den(5); // denominator is now 5
```
The **reduce method** reduces a fraction. A new fraction instance is returned from this method. 
```rust
pub fn reduce(&self) -> Fraction
```
**Example:**
```rust
use matrix_lib::fraction::{Fraction, fraction};
let f: Fraction = fraction(4,6);
let f_reduced: Fraction = f.reduce(); // f_reduced is now 2/3
```
The **pow method** raises a fraction to an integer power. The result is in its reduced form. Any negatives
in the denominator are moved to the numerator. A new fraction instance is returned from this method.
```rust
pub fn pow(&self, exp: isize) -> Fraction
```
**Example:**
```rust
use matrix_lib::fraction::{Fraction, fraction};
let f: Fraction = fraction(4,6);
let f_pow: Fraction = f.pow(-2); // f_pow is 9/4
```
#### 2.1.2. Fraction Class Arithmetic Operator Overloads
The binary '+', '-', '*', '/', and unary '-' arithmetic operators are overloaded to allow more natural 
syntax for performing arithmetic operations on fractions. A new fraction instance is returned from the 
overloads. The result is in its reduced form. Any negatives in the denominator are moved to the numerator.

**Example:**
```rust
use matrix_lib::fraction::{Fraction, fraction};
let f1: Fraction = fraction(4,6);
let f2: Fraction = fraction(-2,3);
let f_add: Fraction = f1 + f2; // adding fractions
let f_sub: Fraction = f1 - f2; // subtracting fractions
let f_mul: Fraction = f1 * f2; // multiplying fractions
let f_div: Fraction = f1 / f2; // dividing fractions
let f_neg: Fraction = -f1;     // negating fractions
```
#### 2.1.3. Fraction Class Comparison Operator Overloads
The binary '==', '!=', '<', '<=', '>', and '>=' comparison operators are overloaded to allow for 
more natural syntax for performing comparison operations between fractions.

**Example:**
```rust
use matrix_lib::fraction::{Fraction, fraction};
let f1: Fraction = fraction(4,6);
let f2: Fraction = fraction(-2,3);
let f_equals: bool = f1 == f2;            // false
let f_nequals: bool = f1 != f2;           // true
let f_lessthan: bool = f1 < f2;           // false
let f_lessthanequals: bool = f1 <= f2;    // false
let f_greaterthan: bool = f1 > f2;        // true
let f_greaterthanequals: bool = f1 >= f2; // true
```
#### 2.1.4. Printing Fractions to Stdout
Fractions can be printed to stdout using the println! macro.

**Example:**
```rust
use matrix_lib::fraction::{Fraction, fraction};
let f: Fraction = fraction(4,6);
println!("The fraction f is: {}", f); // printing to stdout
```

### 2.2. Complex Class
The "complex" class is a data type for representing complex numbers. 

Note: Rust does not have a "class" keyword. Instead, classes can be simulated by using a struct
for storing data and using method syntax to allow function calls on the struct using the dot "." 
notation (i.e. struct.method()). Access to the struct fields must be done using the getter and 
setter methods. This is to prevent the possibility of making inadvertent changes to the complex number.

Data is stored in the following struct:
```rust
pub struct Complex {
  real: f64,
  imag: f64,
}
```
Since execution speed is a lower priority than calculation accuracy, doubles precision floats are 
used to represent the real and imaginary parts instead of single precision floats.

#### 2.2.1 Complex Class Methods
In the following examples, be sure to add "extern crate matrix_lib" to the source code.

The **complex constructor** creates an instance of a complex number. It will accept any valid double 
precision number for both the real and imaginary parts.
```rust
pub fn complex(r: f64, i: f64) -> Complex
```
**Example:**
```rust
use matrix_lib::complex::{Complex, complex};
let c:Complex = complex(3.62,4.88); // create complex number 3.62+4.88j
```
The **getters** allow access to either the real part or imaginary part.
```rust
pub fn re(&self) -> f64
pub fn im(&self) -> f64
```
**Example:**
```rust
use matrix_lib::complex::{Complex, complex};
let c: Complex = complex(3.62,4.88);
let r: f64 = c.re();
let i: f64 = c.im();
```
The **setters** allow modification of the real part or imaginary part. 
```rust
pub fn set_re(&mut self, r: f64)
pub fn set_im(&mut self, i: f64)
```
**Example:**
```rust
use matrix_lib::complex::{Complex, complex};
let c: Complex = complex(3.62,4.88);
c.set_re5.89);   // real part is now 5.89
c.set_im(-9.81); // imaginary part is now -9.81
```
The **conjugate method** returns a new instance of a complex number which is the complex conjugate 
(opposite sign of imaginary part) of the original complex number. 
```rust
pub fn conjugate(&self) -> Complex
```
**Example:**
```rust
use matrix_lib::complex::{Complex, complex};
let c: Complex = complex(3.62,4.88);      // c is 3.62+4.88j
let c_conjugate: Complex = c.conjugate(); // c_conjugate is now 3.62-4.88j
```
The **magnitude method** returns the magnitude of a complex number. The magnitude can be calculated
by taking the square root of the sum of the real and imaginary parts squared. 
(sqrt(real^2 + imaginary^2))
```rust
pub fn mag(&self) -> f64
```
**Example:**
```rust
use matrix_lib::complex::{Complex, complex};
let c: Complex = complex(3.62,4.88); 
let m: f64 = c.mag(); // m is approximately 6.0761
```
The **angle method** returns the angle (radians) of a complex number. The angle can be calculated 
by taking the inverse tangent of the imaginary part divided by the real part. 
(atan2(imaginary/real))
```rust
pub fn angle(&self) -> f64
```
**Example:**
```rust
use matrix_lib::complex::{Complex, complex};
let c: Complex = complex(3.62,4.88); 
let a: f64 = c.angle(); // a is approximately 0.9326 radians
```
The **pow method** raises a complex number to an integer power and returns a new instance of a complex 
number as the result.  i32 is used because the built in Rust powi() method for double precision 
floating points requires an i32 to represent the exponent.
```rust
pub fn pow(&self, exp: i32) -> Complex
```
**Example:**
```rust
use matrix_lib::complex::{Complex, complex};
let c: Complex = complex(3.62,4.88);
let c_pow: Complex = c.pow(2); // c_pow is approximately -10.71+35.33j
```
The **exp method** raises 'e' to the power of the complex number and returns a new instance of a complex 
number as the result.
```rust
pub fn exp(&self) -> Complex
```
**Example:**
```rust
use matrix_lib::complex::{Complex, complex};
let c: Complex = complex(3.62,4.88);
let c_exp: Complex = c.exp(); // c_exp is approximately 6.23-36.81j
```
#### 2.2.2. Complex Class Arithmetic Operator overloads
The binary '+', '-', '*', '/', and unary '-' are overloaded to allow more natural syntax for
performing arithmetic operations on complex numbers.

Note: operator overloads call a hidden method, "delnegzero()", which removes negative zeros from the 
complex number. This is to prevent comparison errors 
Any zero value as a result of an arithmetic operation will always be positive. 
On systems which do not implement negative zeros, this will have no effect.

**Example:**
```rust
use matrix_lib::complex::{Complex, complex};
let c1: Complex = complex(3.62,4.88);
let c2: Complex = complex(-2.65,3.97);
let c_add: Complex = c1 + c2; // adding complex numbers
let c_sub: Complex = c1 - c2; // subtracting complex numbers
let c_mul: Complex = c1 * c2; // multiplying complex numbers
let c_div: Complex = c1 / c2; // dividing complex numbers
let c_neg: Complex = -c1;     // negating complex numbers
```
#### 2.2.3. Complex Class Comparison Operator Overloads
The binary '==' and '!=' comparison operators are overloaded to allow for more natural syntax for performing 
comparison operations between complex numbers. The '<', '<=', '>', and '>=' operators are not overloaded 
as it does not make sense to do those comparisons between complex numbers.

Note: due to rounding errors causing a direct '==' or '!=' comparison between doubles to frequently give 
erroneous comparison results, a value called "COMPLEX_TOL" is set to check if two double precision floats 
are approximately equal. The "COMPLEX_TOL" is set to 0.0001 as a default and can be changed in the source 
code.

**Example:**
```rust
use matrix_lib::complex::{Complex, complex};
let c1: Complex = complex(3.6222,4.8888);
let c2: Complex = complex(3.6221,4.8889);
let c_equals: bool = c1 == c2;            // true - based on the default COMPLEX_TOL value of 0.0001
let c_nequals: bool = c1 != c2;           // false
```
#### 2.2.4. Printing Complex Numbers to Stdout
Complex numbers can be printed to stdout using the println! macro.

**Example:**
```rust
use matrix_lib::complex::{Complex, complex};
let c: Complex = complex(3.62,4.88);
println!("The complex c is: {}", c); // printing to stdout
```

### 2.3. Matrix Class Generic
The "Matrix" class is a data type for representing matrices. The elements are implemented as a 
generic type \<T\>. The type is bound to data types which implement arithmetic operator overloads, 
and the Copy trait.

Note: Rust does not have a "class" keyword. Instead, classes can be simulated by using a struct
for storing data and using method syntax to allow function calls on the struct using the dot "." 
notation (i.e. struct.method()). Access to the struct fields must be done using the getter and 
setter methods. This is to prevent the possibility of making inadvertent changes to the matrices.

Data is stored in the following struct:
```rust
pub struct Matrix<T> {
  num_rows: usize,
  num_columns: usize,
  values: Vec<T>,
}
```
The dimensions of the vector are stored in usize integers. The elements are stored in a Rust library 
implmentation of vectors. Since the number of elements in a matrix is unknown at compile time, an array
is cannot be used to store the values. On the other hand, vectors can be resized and do not need to 
have their sizes defined at compile time.

#### 2.3.1. Matrix Class Methods 
In the following examples, be sure to add "extern crate matrix_lib" to the source code.

The **Matrix constructor** creates an instance of a matrix object. If the rows or columns is zero, the 
constructor will panic with an error message. The size of a matrix cannot be modified. After constructing
the matrix, each element in the matrix is set to a default of 0. 

Note: Matrix variables **must be mutable**, otherwise the element values cannot be changed.
```rust
pub fn matrix<T>(r: usize, c: usize) -> Matrix<T>
```
**Example:**
```rust
use matrix_lib::matrix::{Matrix, matrix}
let mut m: Matrix<f64> = matrix(2,2); // create a 2x2 matrix
```
The **getters** allow access to the dimensions of the matrix and a particular element in the matrix. 
Element indices are zero-based (i.e. the element in the first row and first column are at indices 0,0).
```rust
 pub fn rows(&self) -> usize
 pub fn columns(&self) -> usize
 pub fn get(&self, r: usize, c: usize) -> T
```
**Example:**
```rust
use matrix_lib::matrix::{Matrix, matrix}
let mut m: Matrix<f64> = matrix(2,2); // create a 2x2 matrix
m.set(0,0,1.0); m.set(0,1,2.0);
m.set(1,0,3.0); m.set(1,1,4.0);
let r: usize = m.rows();    // returns 2
let c: usize = m.columns(); // returns 2
let val: f64 = m.get(0,1);  // returns value of element in the 1st row, 2nd column which is 2.0
```
The **setters** allow modification of the elements in the matrix.
```rust
pub fn set(&mut self, r: usize, c: usize, x: T)
```
**Example:**
```rust
use matrix_lib::matrix::{Matrix, matrix}
let mut m::Matrix<f64> = matrix(2,2); // create a 2x2 matrix
m.set(0,0,1.0); m.set(0,1,2.0); // Set 1st row elements to be 1.0, 2.0
m.set(1,0,3.0); m.set(1,1,4.0); // Set 2nd row elements to be 3.0, 4.0
```
The **scale method** multiplies each element in the matrix by a scaling factor. The scaling factor 
must be the same data type as the elements in the matrix. If the scaling factor is a different 
data type, the method will panic about mismatch for multiplying between different data types.
```rust
pub fn scale(&self, s: T) -> Matrix<T>
```
**Example:**
```rust
use matrix_lib::matrix::{Matrix, matrix}
let mut m: Matrix<f64> = matrix(2,2); // create a 2x2 matrix
m.set(0,0,1.0); m.set(0,1,2.0);
m.set(1,0,3.0); m.set(1,1,4.0);
let s: f64 = 2.0;    
let m_scaled: matrix<f64> = m.scale(s); // multiplies each element in the matrix by 2.0
```
The **minor method** returns the matrix of size (n-1)x(n-1) by eliminating the row and column specified.
The row and column indices are zero-based (i.e. the 1st row is index 0, the 1st column is index 0). A
new instance of a matrix is returned. The method will panic if indices greater than the size of the matrix
are input.
```rust
pub fn minor(&self, r: usize, c: usize) -> Matrix<T>
```
**Example:**
```rust
use matrix_lib::matrix::{Matrix, matrix}
let mut m::Matrix<f64> = matrix(2,2); // create a 2x2 matrix
m.set(0,0,1.0); m.set(0,1,2.0);
m.set(1,0,3.0); m.set(1,1,4.0);
let m: Matrix<f64> = m.minor(0,0);  // returns value of element in the 1st row, 2nd column which is 2.0
```
The **det method** calculates the determinant of a matrix and returns a scalar of the same data type as 
the elements in the matrix. The matrix must be square (i.e. number of rows equals the number of columns).
```rust
pub fn det(&self) -> T
```
**Example:**
```rust
use matrix_lib::matrix::{Matrix, matrix}
let mut m: Matrix<f64> = matrix(2,2); // create a 2x2 matrix
m.set(0,0,1.0); m.set(0,1,2.0);
m.set(1,0,3.0); m.set(1,1,4.0);
let d: f64 = m.det();  // returns -2.0
```
The **inverse method** calculates the inverse of an nxn matrix and returns a new matrix instance as
the inverse matrix. The matrix must be square, and its determinant must be nonzero, to be invertible.
```rust
pub fn inverse(&self) -> Matrix<T>
```
**Example:**
```rust
use matrix_lib::matrix::{Matrix, matrix}
let mut m: Matrix<f64> = matrix(2,2); // create a 2x2 matrix
m.set(0,0,1.0); m.set(0,1,2.0);
m.set(1,0,3.0); m.set(1,1,4.0);
let m_inv: matrix<f64> = m.inverse();  // returns 2x2 inverse matrix
```
#### 2.3.2. Matrix Class Arithmetic Operator Overloads
The binary '+', '-', and '*' operators are overloaded to allow more natural syntax for performing 
arithmetic operations on matrices.

**Example:**
```rust
use matrix_lib::matrix::{Matrix, matrix}
let m1: Matrix<i32> = matrix(2,2);
let m2: Matrix<i32> = matrix(2,2);
m1.set(0,0,1); m1.set(0,1,2);
m1.set(1,0,3); m1.set(1,1,4); // set values in m1
m2.set(0,0,4); m2.set(0,1,3);
m2.set(1,0,2); m2.set(1,1,1); // set values in m2
let m_add: Matrix<i32> = m1.clone() + m2.clone(); // adding matrices
let m_sub: Matrix<i32> = m1.clone() - m2.clone(); // subtracting matrices
let m_mul: Matrix<i32> = m1.clone() * m2.clone(); // multiplying matrices
```
Note the use of the clone method for arithmetic operations. The arithmetic operators consume 
the operands because they are pass by value. Rust's default implementation for overloading 
arithmetic operators requires them to be pass by value.

#### 2.3.3. Matrix Class Comparison Operator Overloads
The binary '==' and '!=' comparison operators are overloaded to allow more natural syntax for performing
comparisons between matrices. The '<', '<=', '>', and '>=' operators are not overloaded 
as it does not make sense to do those comparisons between matrices.

**Example:**
```rust
use matrix_lib::matrix::{Matrix, matrix}
let m1: Matrix<i32> = matrix(2,2);
let m2: Matrix<i32> = matrix(2,2);
m1.set(0,0,1); m1.set(0,1,2);
m1.set(1,0,3); m1.set(1,1,4); // set values in m1
m2.set(0,0,4); m2.set(0,1,3);
m2.set(1,0,2); m2.set(1,1,1); // set values in m2
let m_equals: bool = m1 == m2; // false
let m_nequals: bool = m1 != m2; // true
```
Note the omission of the clone method. The comparison operators take operands by reference instead of 
by value. The operands are not consumed after the comparison.

#### 2.3.4. Printing Matrices to Stdout
Matrices can be printed to stdout using the println! macro.

**Example:**
```rust
use matrix_lib::matrix::{Matrix, matrix};
let m: Matrix<f64> = matrix(2,2);
m.set(0,0,1.0); m.set(0,1,2.0);
m.set(1,0,3.0); m.set(1,1,4.0);
println!("The matrix m is : {}", m); // printing to stdout
```

### 2.4. Discrete Fourier Transform (DFT) Class
The "DFT" class is a data type for representing a set of matrices for computing the discrete Fourier
transform on a vector of complex numbers. See 
https://en.wikipedia.org/wiki/Discrete_Fourier_transform for more information on the discrete 
Fourier transform, the transform, unitary, and inverse matrices.

Note: Rust does not have a "class" keyword. Instead, classes can be simulated by using a struct
for storing data and using method syntax to allow function calls on the struct using the dot "." 
notation (i.e. struct.method()). Access to the struct fields must be done using the getter and 
setter methods. This is to prevent the possibility of making inadvertent changes to the transform 
matrices.

Data is stored in the following struct:
```rust
use ::std::collections::HashMap;
use complex;
use matrix;
pub struct DFT {
  num_pts: usize,
  cache: HashMap<String, matrix::Matrix<complex::Complex>>,
}
```
A hashmap is used to cache the computed transform matrices for reuse. The transform matrices depend on 
the num_pts field. A new DFT must be created if there are a different number of points in the input 
vector. Notice the transform matrices can only contain complex numbers defined in the complex class.

#### 2.4.1. DFT Class methods
In the following examples, be sure to add "extern crate matrix_lib" to the source code.

The **DFT constructor** creates an instance of a DFT object. If the number of points is 0, it will panic.
```rust
pub fn dft(n: usize) -> DFT
```
**Example:**
```rust
use matrix_lib::DFT::{DFT, dft};
let d: DFT = dft(4);
```
The **getter** allows access to the number of points to compute a DFT for.
```rust
pub fn npts(&self) -> usize
```
**Example:**
```rust
use matrix_lib::DFT::{DFT, dft};
let d: DFT = dft(4);
let n_pts: usize = dft.npts(); // returns 4
```
The **transform_matrix method** calculates the n point discrete Fourier transform matrix. If the transform 
matrix is not in the cache, it calculates the transform matrix, stores it in the cache, and returns a clone
of the transform matrix. If the transform matrix is in the cache, it retrieves the matrix from the cache 
and returns a clone of the transform matrix. 
```rust
use complex;
use matrix;
pub fn transform_matrix(&mut self) -> matrix::Matrix<complex::Complex>
```
**Example:**
```rust
use matrix_lib::DFT::{DFT, dft};
use matrix_lib::matrix::{Matrix, matrix}
use matrix_lib::complex::{Complex, complex}
let d: DFT = dft(4);
let tfm_mat: matrix::Matrix<complex::Complex> = dft.transform_matrix(); // returns 4x4 matrix
```
The **unitary_matrix method** calculates the n point discrete Fourier transform unitary matrix. If the unitary 
matrix is not in the cache, it calculates the unitary matrix, stores it in the cache, and returns a clone
of the unitary matrix. If the unitary matrix is in the cache, it retrieves the matrix from the cache 
and returns a clone of the unitary matrix. 
```rust
use complex;
use matrix;
pub fn unitary_matrix(&mut self) -> matrix::Matrix<complex::Complex>
```
**Example:**
```rust
use matrix_lib::DFT::{DFT, dft};
use matrix_lib::matrix::{Matrix, matrix}
use matrix_lib::complex::{Complex, complex}
let d: DFT = dft(4);
let uni_mat: matrix::Matrix<complex::Complex> = dft.unitary_matrix(); // returns 4x4 matrix
```
The **inverse_matrix method** calculates the n point discrete Fourier transform inverse matrix. If the inverse 
matrix is not in the cache, it calculates the inverse matrix, stores it in the cache, and returns a clone
of the inverse matrix. If the inverse matrix is in the cache, it retrieves the matrix from the cache 
and returns a clone of the inverse matrix.
```rust
use complex;
use matrix;
pub fn inverse_matrix(&mut self) -> matrix::Matrix<complex::Complex>
```
**Example:**
```rust
use matrix_lib::DFT::{DFT, dft};
use matrix_lib::matrix::{Matrix, matrix}
use matrix_lib::complex::{Complex, complex}
let d: DFT = dft(4);
let inv_mat: matrix::Matrix<complex::Complex> = dft.inverse_matrix(); // returns 4x4 inverse matrix
```
The **dft_tfm method** calculates the discret Fourier transform for vector with n points and returns a 
vector with the same dimensions. The vector must have elements of the complex number type defined in 
this matrix library.
```rust
use complex;
use matrix;
pub fn dft_tfm(&mut self, vector: matrix::Matrix<complex::Complex>) -> matrix::Matrix<complex::Complex>
```
**Example:**
```rust
use matrix_lib::DFT::{DFT, dft};
use matrix_lib::matrix::{Matrix, matrix}
use matrix_lib::complex::{Complex, complex}
let d: DFT = dft(4);
let v: matrix::Matrix<complex::Complex> = matrix(4,1); 
/* set values of the vector */
let v_tfm: matrix::Matrix<complex::Complex> = d.dft_tfm(v); // returns DFT of the vector v
```
The **dft_inv method** calculates the inverse discret Fourier transform for vector with n points and returns a 
vector with the same dimensions. The vector must have elements of the complex number type defined in 
this matrix library.
```rust
use complex;
use matrix;
pub fn dft_inv(&mut self, vector: matrix::Matrix<complex::Complex>) -> matrix::Matrix<complex::Complex>
```
**Example:**
```rust
use matrix_lib::DFT::{DFT, dft};
use matrix_lib::matrix::{Matrix, matrix}
use matrix_lib::complex::{Complex, complex}
let d: DFT = dft(4);
let v: matrix::Matrix<complex::Complex> = matrix(4,1); 
/* set values of the vector */
let v_inv: matrix::Matrix<complex::Complex> = d.dft_inv(v); // returns inverse DFT of the vector v
```
## 3. Further Improvements

## 4. Test Notes
* All code has been tested on Windows 10 Pro 64-bit.
* rust version 1.13.0 was used to compile and test the code. 
* cargo version 0.13.0-nightly was used as the build system and package managers.

## Rust vs. C++

### Rust vs. C++ differences
* Rust defaults to not allow variables to be changed. Must use "mut" keyword to allow new values to be assigned to a variable. C++ defaults to allow new values to be assigned to variable. C++ uses "const" to keep a variable constant.
* Rust defaults to private properties/methods in a module definition. C++ defaults to public properties/methods in a class definition.

### Rust and C++ similarities
* Rust uses "cargo" similar to a C++ "makefile" for easier management of code compilation. "Cargo" also includes package managemnet features.

### Rust quirks
* Cannot overload the '=' operator in Rust
* Overloading functions is hard and tedious. Also, "overloaded" functions cannot accept different number of input arguments.

### Rust testing/debugging
* cargo test -- --nocapture shows stdout for all tests (even ones that pass)
