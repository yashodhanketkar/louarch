NAME := louarch

.PHONY: all build run test clean
default: help

build:
	@echo "Building Rust project..."
	cargo build

release:
	@echo "Building Rust project in release mode..."
	cargo build --release

run: build
	@echo "Running the project..."
	./target/debug/$(NAME)

test:
	@echo "Running tests..."
	cargo test

clean:
	@echo "Cleaning build artifacts..."
	cargo clean

docs:
	@echo "Building documentation..."
	cargo doc --no-deps --document-private-items

install: release
	@echo "Installing the project..."
	./scripts/install.sh

uninstall:
	@echo "Uninstalling the project..."
	./scripts/uninstall.sh

help:
	@echo "Usage: make [target]"
	@echo "      all: Build the project"
	@echo "    build: Build the project"
	@echo "  release: Build the project in release mode"
	@echo "      run: Run the project"
	@echo "     test: Run the tests"
	@echo "    clean: Clean the build artifacts"
	@echo "     docs: Build the documentation"
	@echo "     Install: Install the project"
	@echo "   uninstall: Uninstall the project"
