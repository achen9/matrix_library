### To compile hw9 code
Make sure you have navigated to the /hw9/ folder as the current directory. Make sure a "Cargo.toml" file is in there.
* Type "cargo build" to compile all code. The compiled code will be placed in the "target" directory.

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

