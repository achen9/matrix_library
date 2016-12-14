# Matrix Library Documentation
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

### To compile hw9 code
Make sure you have navigated to the /hw9/ folder as the current directory. Make sure a "Cargo.toml" file is in there.
* Type "cargo build" to compile all code. The compiled code will be placed in the "target" directory.
* Type "cargo test" to run the unit tests.
* Type "cargo run" to run the code in the main.rs file. The main.rs file contains a benchmark to test the speed of the matrix library implementation.

### To run hw9 code
Make sure you have navigated to the /hw9/ folder as the current directory.
* If an error message pops up, try the suggested fix if one is mentioned.
* All programs perform some basic input error checking and will recommend a fix if the input error is detected.  
* Type "cargo run" to run the code.

### Test notes
* All code has been tested on a Raspberry Pi 3 Model B running Raspbian.
* rust version 1.13.0 was used to compile and test the code. 
* cargo version 0.13.0 was used as the build system and package managers.

## Rust vs. C++
Alex Chen, 12/15/2016

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
