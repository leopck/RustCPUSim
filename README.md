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
