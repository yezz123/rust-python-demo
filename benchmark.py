#!/usr/bin/env python3
"""
Performance Benchmark Script for Rust-Python Calculator

This script compares the performance of basic arithmetic operations
between pure Python and Rust implementations. It generates random numbers
and measures the execution time for both implementations to demonstrate
the performance benefits of using Rust for computationally intensive tasks.
"""

import time
import random
from rust_calculator import add as rust_add, multiply as rust_multiply


def python_add(a: float, b: float) -> float:
    """Pure Python implementation of addition."""
    return a + b


def python_multiply(a: float, b: float) -> float:
    """Pure Python implementation of multiplication."""
    return a * b


def benchmark(
    name: str, python_func: callable, rust_func: callable, iterations: int = 1000000
) -> None:
    """
    Run a performance benchmark comparing Python and Rust implementations.

    Args:
        name: Name of the operation being benchmarked
        python_func: Python implementation of the operation
        rust_func: Rust implementation of the operation
        iterations: Number of operations to perform (default: 1,000,000)
    """
    # Generate random numbers for testing
    numbers = [
        (random.random() * 100, random.random() * 100) for _ in range(iterations)
    ]

    # Benchmark Python implementation
    start = time.time()
    for a, b in numbers:
        python_func(a, b)
    python_time = time.time() - start

    # Benchmark Rust implementation
    start = time.time()
    for a, b in numbers:
        rust_func(a, b)
    rust_time = time.time() - start

    # Print results
    print(f"Operation: {name}")
    print(f"Python time: {python_time:.4f} seconds")
    print(f"Rust time:   {rust_time:.4f} seconds")
    print(f"Speedup:     {python_time / rust_time:.2f}x")
    print()


if __name__ == "__main__":
    print("Performance comparison between Python and Rust implementations")
    print("=============================================================")

    # Run benchmarks for basic arithmetic operations
    benchmark("Addition", python_add, rust_add)
    benchmark("Multiplication", python_multiply, rust_multiply)
