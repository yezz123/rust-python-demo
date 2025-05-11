//! Rust-Python Calculator Module
//!
//! This module implements a calculator with basic arithmetic operations in Rust,
//! exposed to Python through PyO3 bindings. It provides both a class-based interface
//! and direct function calls for maximum flexibility.

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

/// A calculator class that maintains state in memory
///
/// This struct is exposed to Python as a class with methods for various
/// arithmetic operations. It maintains an internal memory value that can be
/// modified through its methods.
#[pyclass]
struct Calculator {
    /// Internal memory value for the calculator
    memory: f64,
}

#[pymethods]
impl Calculator {
    /// Creates a new calculator instance with memory initialized to zero
    #[new]
    fn new() -> Self {
        Calculator { memory: 0.0 }
    }

    /// Add a number to the calculator memory
    ///
    /// # Arguments
    /// * `value` - The number to add to the current memory value
    ///
    /// # Returns
    /// The new memory value after addition
    fn add(&mut self, value: f64) -> PyResult<f64> {
        self.memory += value;
        Ok(self.memory)
    }

    /// Subtract a number from the calculator memory
    ///
    /// # Arguments
    /// * `value` - The number to subtract from the current memory value
    ///
    /// # Returns
    /// The new memory value after subtraction
    fn subtract(&mut self, value: f64) -> PyResult<f64> {
        self.memory -= value;
        Ok(self.memory)
    }

    /// Multiply the calculator memory by a number
    ///
    /// # Arguments
    /// * `value` - The number to multiply the current memory value by
    ///
    /// # Returns
    /// The new memory value after multiplication
    fn multiply(&mut self, value: f64) -> PyResult<f64> {
        self.memory *= value;
        Ok(self.memory)
    }

    /// Divide the calculator memory by a number
    ///
    /// # Arguments
    /// * `value` - The number to divide the current memory value by
    ///
    /// # Returns
    /// The new memory value after division
    ///
    /// # Errors
    /// Returns a PyValueError if attempting to divide by zero
    fn divide(&mut self, value: f64) -> PyResult<f64> {
        if value == 0.0 {
            return Err(PyValueError::new_err("Division by zero"));
        }
        self.memory /= value;
        Ok(self.memory)
    }

    /// Compute the power of the memory value
    ///
    /// # Arguments
    /// * `exponent` - The power to raise the current memory value to
    ///
    /// # Returns
    /// The new memory value after exponentiation
    fn power(&mut self, exponent: f64) -> PyResult<f64> {
        self.memory = self.memory.powf(exponent);
        Ok(self.memory)
    }

    /// Calculate the square root of the memory value
    ///
    /// # Returns
    /// The new memory value after taking the square root
    ///
    /// # Errors
    /// Returns a PyValueError if attempting to take the square root of a negative number
    fn sqrt(&mut self) -> PyResult<f64> {
        if self.memory < 0.0 {
            return Err(PyValueError::new_err("Cannot take square root of negative number"));
        }
        self.memory = self.memory.sqrt();
        Ok(self.memory)
    }

    /// Get the current value stored in memory
    ///
    /// # Returns
    /// The current memory value
    fn get_value(&self) -> f64 {
        self.memory
    }

    /// Reset the calculator memory to zero
    ///
    /// # Returns
    /// The memory value after reset (always 0.0)
    fn clear(&mut self) -> f64 {
        self.memory = 0.0;
        self.memory
    }
}

// Individual math operations for direct use in Python
// These functions are exposed directly to Python without the class wrapper

/// Add two numbers
#[pyfunction]
fn add(a: f64, b: f64) -> f64 {
    a + b
}

/// Subtract b from a
#[pyfunction]
fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

/// Multiply two numbers
#[pyfunction]
fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// Divide a by b
///
/// # Errors
/// Returns a PyValueError if attempting to divide by zero
#[pyfunction]
fn divide(a: f64, b: f64) -> PyResult<f64> {
    if b == 0.0 {
        return Err(PyValueError::new_err("Division by zero"));
    }
    Ok(a / b)
}

/// Calculate a raised to the power of b
#[pyfunction]
fn power(base: f64, exponent: f64) -> f64 {
    base.powf(exponent)
}

/// The Python module definition
///
/// This function is called when the module is imported in Python.
/// It registers all the classes and functions that should be available
/// to Python code.
#[pymodule]
fn rust_calculator(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register the Calculator class
    m.add_class::<Calculator>()?;

    // Register individual functions
    m.add_function(wrap_pyfunction!(add, _py)?)?;
    m.add_function(wrap_pyfunction!(subtract, _py)?)?;
    m.add_function(wrap_pyfunction!(multiply, _py)?)?;
    m.add_function(wrap_pyfunction!(divide, _py)?)?;
    m.add_function(wrap_pyfunction!(power, _py)?)?;

    Ok(())
}
