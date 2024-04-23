# Simple CPU Simulator

This repository contains a Rust-based simulation of a simple CPU architecture that demonstrates basic operations such as load, store, add, subtract, jump, and halt. The CPU simulation is designed to be a minimalistic educational tool for understanding how a CPU works at a basic level.

## Overview

The simulator is built around a `CPU` struct which represents a CPU with an accumulator, a program counter (`pc`), and a fixed-size memory. It supports a set of instructions encoded as an enum `Instruction`. The CPU executes these instructions, modifying its internal state accordingly.

## Features

- **CPU Operations**: Simulates basic CPU operations including loading, storing, addition, subtraction, jumping, and halting.
- **Memory Management**: Features a simple memory model with direct access based on instruction arguments.
- **Program Execution**: Ability to load and execute user-defined programs encoded directly in memory.

## Getting Started

### Prerequisites

Ensure you have Rust installed on your machine. To install Rust, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

### Installation

Clone this repository to your local machine using:

```bash
git clone https://github.com/your-github-username/simple-cpu-simulator.git
cd simple-cpu-simulator
```


### Running the simulator

```bash

cargo build
cargo run

```

## Usage

The main program in `main.rs` demonstrates loading a simple program into the CPU's memory and executing it. You can modify this program or create new ones by adjusting the instruction and data values in the array passed to the `load_program` method.

## Example Program

Here's a quick breakdown of the sample program included in the source code:

- **LOAD** the value from memory address `10` into the accumulator.
- **ADD** the value at memory address `11` to the accumulator.
- **STORE** the accumulator's value into memory address `12`.
- **HALT** the execution.

## Contributing

Contributions are welcome! Please feel free to submit pull requests, report bugs, or suggest features.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
```

You can adjust the repository URL and any specific details to better fit your project's documentation needs.
