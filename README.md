# Serial-MIDI

Serial-MIDI is a barebones Rust program that allows you to read input from a serial port and send it to a MIDI device.

## Why?

As of February 2024, the Arduino libraries do not provide support for MIDIUSB on the Arduino Giga microcontroller. As a temporary solution, I chose to write this program using a basic serial connection instead.

## Usage

To use Serial-MIDI, you need to have Rust installed on your system. Once you have Rust installed, you can clone the repository and build the program using Cargo. After building, you can run the program and specify the serial port and MIDI device to use.

## Features

- Read input from a serial port
- Route serial port input to a MIDI device
- Specify the serial port and MIDI device to use
- Simple and efficient implementation
- Cross-platform support (tested on MacOS)

## Getting Started

To get started with Serial-MIDI, follow these steps:

1. Install Rust on your system e.g. by following [this guide](https://www.rust-lang.org/tools/install).
2. Clone the Serial-MIDI repository.
3. Build the program using Cargo.
4. Run the program and specify the serial port and MIDI device.

## Contributing

Contributions are welcome! If you have any ideas, suggestions, or bug reports, please open an issue or submit a pull request on the GitHub repository.

## License

Serial-MIDI is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.
