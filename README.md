# Rust-Python Calculator Demo

This project demonstrates how to combine Rust and Python to create high-performance Python extensions. It implements a simple calculator with basic arithmetic operations, showing both the Rust implementation and Python bindings.

## Features

- Basic arithmetic operations (add, subtract, multiply, divide)
- Advanced operations (power, square root)
- Memory-based calculator functionality
- Performance benchmarking between Rust and Python implementations

## Prerequisites

- Rust (latest stable version)
- Python 3.9+
- maturin (Python package for building Rust extensions)

## Installation

1. Install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install maturin:

```bash
pip install maturin
```

3. Clone the repository:

```bash
git clone https://github.com/yezz123/rust-python-demo.git

cd rust-python-de
```

4. Build the Rust extension:

```bash
maturin develop
```

## Usage

### Basic Calculator Operations

```python
from rust_calculator import Calculator

# Create a new calculator instance
calc = Calculator()

# Basic operations
result = calc.add(5.0)      # Add 5
result = calc.multiply(2.0)  # Multiply by 2
result = calc.subtract(3.0)  # Subtract 3
result = calc.divide(2.0)    # Divide by 2

# Advanced operations
result = calc.power(2.0)     # Square the current value
result = calc.sqrt()         # Take square root

# Get current value
current = calc.get_value()

# Reset calculator
calc.clear()
```

### Direct Function Calls

```python
from rust_calculator import add, subtract, multiply, divide, power

# Use individual functions
result = add(5.0, 3.0)
result = multiply(4.0, 2.0)
result = power(2.0, 3.0)  # 2^3
```

## Running Benchmarks

The project includes a benchmark script that compares the performance of Rust and Python implementations:

```bash
python benchmark.py
```

This will run performance comparisons for basic arithmetic operations between Rust and Python implementations.

## Project Structure

```sh
rust-python-demo/
├── src/
│   └── lib.rs           # Rust implementation
├── python/
│   └── rust_calculator/ # Python package
├── tests/               # Test files
├── benchmark.py         # Performance benchmarks
├── Cargo.toml          # Rust dependencies
└── pyproject.toml      # Python package configuration
```

## How It Works

1. The Rust code is compiled into a Python extension using PyO3 and maturin
2. The extension provides both a class-based interface (Calculator) and direct function calls
3. Python code can import and use the Rust functions as if they were native Python code
4. The benchmark script demonstrates the performance difference between Rust and Python implementations
