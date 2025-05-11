.PHONY: setup install test lint format clean build benchmark

# Python virtual environment
VENV = .venv
PYTHON = $(VENV)/bin/python
PIP = $(VENV)/bin/pip

# Default target
all: setup install

# Setup development environment
setup:
	@echo "Setting up development environment..."
	python3 -m venv $(VENV)
	$(PIP) install --upgrade pip
	$(PIP) install -r requirements.txt
	$(PIP) install -r requirements-dev.txt
	pre-commit install
	@echo "Development environment setup complete!"

# Install dependencies
install:
	@echo "Installing dependencies..."
	$(PIP) install -e .
	@echo "Dependencies installed!"

# Run tests
test:
	@echo "Running tests..."
	$(PYTHON) -m pytest tests/
	cargo test
	@echo "Tests completed!"

# Run linters
lint:
	@echo "Running linters..."
	$(PYTHON) -m ruff check .
	$(PYTHON) -m ruff format --check .
	cargo clippy
	cargo fmt -- --check
	@echo "Linting completed!"

# Format code
format:
	@echo "Formatting code..."
	$(PYTHON) -m ruff format .
	cargo fmt
	@echo "Formatting completed!"

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	rm -rf build/ dist/ *.egg-info/
	cargo clean
	@echo "Clean completed!"

# Build the project
build:
	@echo "Building project..."
	maturin build
	@echo "Build completed!"

# Run benchmarks
benchmark:
	@echo "Running benchmarks..."
	$(PYTHON) benchmark.py
	@echo "Benchmarks completed!"

# Development mode
dev: setup install
	@echo "Development environment ready!"

# Help target
help:
	@echo "Available targets:"
	@echo "  setup      - Set up development environment"
	@echo "  install    - Install dependencies"
	@echo "  test       - Run tests"
	@echo "  lint       - Run linters"
	@echo "  format     - Format code"
	@echo "  clean      - Clean build artifacts"
	@echo "  build      - Build the project"
	@echo "  benchmark  - Run benchmarks"
	@echo "  dev        - Set up development environment and install dependencies"
