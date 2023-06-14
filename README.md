# esp32_wasm3_blink
LED blink example for esp32 using ESP-IDF and hosted Rust code with Wasm3.

The ESP-IDF project is written in C++, which reads a binary Wasm file. The led blinking is controlled in the Rust program, which has been compiled to a .wasm file. 

The following C++ functions were binded to Rust with Wasm3's m3API to alow LED control from Rust:
* configure_led()
* blink_led()

## Requirements
### ESP-IDF
Follow [this](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/) guide to setup ESP-IDF to build and flash your esp32 board.

### Rust
The hosted code in this example was written in Rust. You do not need to Rust to run this example as it is already compiled as a wasm binary, however if you would like to make any modification to the Rust program you will need to re-compile the program. You can get started with Rust [here](https://www.rust-lang.org/learn/get-started). 

### Wasm3
[Wasm3](https://github.com/wasm3/wasm3) is used to run WebAssembley on embedded devices. Installation instructions can be found [here](https://github.com/wasm3/wasm3/blob/main/docs/Installation.md).

## To Run
Within the top level repository folder run:

`$ idf.py set-target esp32`

`$ idf.py build`

`$ idf.py -p <device-name> flash`  
  
Additional instructions for building ESP-IDF projects can be found [here](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/#build-your-first-project).

## File Structure
TODO
