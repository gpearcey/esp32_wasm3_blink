# esp32_wasm3_blink
LED blink example for esp32 using ESP-IDF and hosted Rust code with Wasm3.

The ESP-IDF project is written in C++, which reads a binary Wasm file. The led blinking is controlled in the Rust program, which has been compiled to a .wasm file. 

The following C++ functions were binded to Rust with Wasm3's m3API to alow LED control from Rust:
* configure_led()
* blink_led()

## Requirements
### ESP-IDF
Follow [this](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/) guide to setup ESP-IDF to build and flash your esp32 board. This poject was completed with esp version 5.0.2. Other versions are likely compatible, but not tested. 

### Rust
The hosted code in this example was written in Rust. You do not need to Rust to run this example as it is already compiled as a wasm binary, however if you would like to make any modification to the Rust program you will need to re-compile the program. You can get started with Rust [here](https://www.rust-lang.org/learn/get-started). 

You will also need the following to compile rust to .wasm files:
* wasm-gc 
  * `$ cargo install wasm-gc`


### Wasm3
Wasm3 is used to run WebAssembley on embedded devices. Installation instructions can be found [here](https://github.com/wasm3/wasm3/blob/main/docs/Installation.md). You will also need to clone the [Wasm3 Repository](https://github.com/wasm3/wasm3). 

## To Run
Update the path to wasm3 in main component's CMakeLists.txt (esp32_wasm3_blink/esp-idf-project/main/CMakeLists.txt) to where you have cloned wasm3.

Update this line with your path to wasm3:

`add_subdirectory($ENV{HOME}/repos/wasm3/source ${build_dir}/m3) #update with your path to wasm3/`

Within the top level repository folder run:

`$ idf.py set-target esp32`

`$ idf.py build`

`$ idf.py -p <device-name> flash`  
  
Additional instructions for building ESP-IDF projects can be found [here](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/#build-your-first-project).

## File Structure
TODO

## Generate .wasm.h files from a rust program to use in ESP_IDF project

Navigate to wasm-project/

`$ cd esp32_wasm3_blink/wasm-project/`

Add wasm as a target

`$ rustup target add wasm32-unknown-unknown`

Compile

`$ wasm-pack build`

Navigation to pkg/

`$ cd pkg/`

Created .wasm.h file that C++ code can parse:

`$ xxd -i wasm_project_bg.wasm > wasm_project_bg.wasm.h`

Replace `wasm_project_bg.wasm` with your .wasm file name
