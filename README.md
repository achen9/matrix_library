# Matrix Library v0.1.0 Documentation
This repository contains a matrix library implementation written in Rust. A simple set of matrix operations
such as arithmetic operations, determinants, inverse, and discrete Fourier transforms are provided.

Author: Alex Chen, alexac9@uw.edu

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
The following sections document the data types, methods, and design decisions associated with the 
contents of the matrix library.

### 2.1. Fraction Class
The "fraction" class is a data type for representing real numbers in rational form. Data is stored in the following
struct:
```rust
struct Fraction {
  numerator: isize,
  denominator: isize,
}
```
#### 2.1.1. Fraction Class Methods
In the following examples, be sure to add "extern create matrix_lib" to the source code.
```rust
fn fraction(n: isize, d: isize) -> Fraction
```
The fraction constructor creates an instance of a fraction. If a 0 is specified for the denominator,
the constructor will panic with an error message.

**Example:**
```rust
use matrix_lib::fraction::{Fraction, fraction};
let f:Fraction = fraction(2,3);
```

```rust
fn num(&self) -> isize
fn den(&self) -> isize
```
The getters allow access to either the numerator or denominator.

**Example:**
```rust
use matrix_lib::fraction::{Fraction, fraction};
let f: Fraction = fraction(2,3);
let n: isize = f.num();
let d: isize = f.den();
```

```rust
fn set_num(&mut self, n: isize)
fn set_den(&mut self, d: isize)
```
The setters allow modification of the numerator or denominator. The set_den() method will panic if 
a 0 is input as the argument.

**Example:**
```rust
use matrix_lib::fraction::{Fraction, fraction};
let f: Fraction = fraction(2,3);
f.set_num(3); // numerator is now 3
f.set_den(5); // denominator is now 5
```

```rust
fn reduce(&self) -> Fraction
```
The reduce method reduces a fraction. 

**Example:**
```rust
use matrix_lib::fraction::{Fraction, fraction};
let f: Fraction = fraction(4,6);
let f_reduced: Fraction = f.reduce(); // f_reduced is now 2/3
```

```rust
fn pow(&self, exp: isize) -> Fraction
```
The pow method raises a fraction to an integer power. The result is in its reduced form.

**Example:**
```rust
use matrix_lib::fraction::{Fraction, fraction};
let f: Fraction = fraction(4,6);
let f_pow: Fraction = f.pow(-2); // f_pow is 9/4
```

#### 2.1.2. Fraction Class Operator overloads
The '+', binary '-', unary '-', '*', and '/' operators are overloaded to allow more natural syntax for
performing arithmetic operations on fractions.

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
#### 2.1.3. Printing Fractions to Stdout
Fractions can be printed to stdout using the println! macro.

**Example:**
```rust
use matrix_lib::fraction::{Fraction, fraction};
let f: Fraction = fraction(4,6);
println!("The fraction f is: {}", f); // printing to stdout
```

### 2.2. Complex Class

### 2.3. Matrix Class Generic

### 2.4. Discrete Fourier Transform (DFT) Class

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
