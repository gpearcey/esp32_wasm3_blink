# esp32_wasm3_blink
LED blink example for esp32 using ESP-IDF and Wasm3.

The ESP-IDF project is written in C++, which reads a binary Wasm file. The led blinking is controlled in the Rust program, which has been compiled to a .wasm file. 

The following C++ functions were binded to Rust with Wasm3's m3API to alow LED control from Rust:
* configure_led()
* blink_led()
* delay()

## Requirements
### ESP-IDF
Follow [this](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/) guide to setup ESP-IDF to build and flash your esp32 board. This poject was completed with esp version 5.0.2. Other versions are likely compatible, but not tested. 

### Rust
The hosted code in this example was written in Rust. You can get started with Rust [here](https://www.rust-lang.org/learn/get-started). 

You will also need the following to compile rust to .wasm files:
* wasm-gc 
  * `$ cargo install wasm-gc`
* wasi-sdk
  * Available from the  [release page](https://github.com/WebAssembly/wasi-sdk/releases)


### Wasm3
Wasm3 is used to run WebAssembley on embedded devices. Installation instructions can be found [here](https://github.com/wasm3/wasm3/blob/main/docs/Installation.md). You will also need to clone the [Wasm3 Repository](https://github.com/wasm3/wasm3). 

## Instructions

Update the path to wasm3 in main component's CMakeLists.txt (esp32_wasm3_blink/esp-idf-project/main/CMakeLists.txt) to where you have cloned wasm3.
Update this line with your path to wasm3: 
`add_subdirectory($ENV{HOME}/repos/wasm3/source ${build_dir}/m3) #update with your path to wasm3/`

### Compile lib.rs
Navigate to top level directory and run

`$ ./build-app.sh`

This compiles the rust program that controls LED to a WebAssembly binary file, then into a hex format that is loaded into main.cpp.

### Flash the esp32
Navigate to esp-idf-project:

`$ cd esp-idf-project/`

Build the program and flash the board:

`$ idf.py set-target esp32`

`$ idf.py build`

`$ idf.py -p <device-name> flash`  
  
Additional instructions for building ESP-IDF projects can be found [here](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/#build-your-first-project).

## File Structure

```
esp32_wasm3_blink
├── esp-idf-project
│   ├── main
│   │   ├── CMakeLists.txt
│   │   ├── linker.lf
|   |   └── main.cpp
│   ├── CMakeLists.txt
│   ├── sdkconfig
│   └── sdkconfig.defaults
├── wasm-project
│   ├── src
|   |   └── lib.rs
│   └── cargo.toml
├── .gitignore
├── build-app.sh
└── README.md
```
