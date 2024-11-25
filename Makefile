.PHONY: all build check test lint format clean help

# Colors for output
GREEN := \033[0;32m
YELLOW := \033[1;33m
RED := \033[0;31m
NC := \033[0m

# Default target
all: help

# Build the project
build:
	@echo "$(YELLOW)Building project...$(NC)"
	cargo build

# Build for release
release:
	@echo "$(YELLOW)Building release version...$(NC)"
	cargo build --release

# Run all checks (format, lint, and test)
check: format-check lint test
	@echo "$(GREEN)All checks passed!$(NC)"

# Run tests
test:
	@echo "$(YELLOW)Running tests...$(NC)"
	cargo test

# Run linter (clippy)
lint:
	@echo "$(YELLOW)Running clippy...$(NC)"
	cargo clippy -- -D warnings

# Check code formatting
format-check:
	@echo "$(YELLOW)Checking code format...$(NC)"
	cargo fmt --all -- --check

# Format code
format:
	@echo "$(YELLOW)Formatting code...$(NC)"
	cargo fmt --all
	@echo "$(GREEN)Code formatting complete!$(NC)"

# Clean build artifacts
clean:
	@echo "$(YELLOW)Cleaning build artifacts...$(NC)"
	cargo clean
	@echo "$(GREEN)Clean complete!$(NC)"

# Install development dependencies
dev-deps:
	@echo "$(YELLOW)Installing development dependencies...$(NC)"
	rustup component add clippy rustfmt
	@echo "$(GREEN)Development dependencies installed!$(NC)"

# Run the project
run:
	@echo "$(YELLOW)Running project...$(NC)"
	cargo run

# Show help
help:
	@echo "Available targets:"
	@echo "  make build         - Build the project"
	@echo "  make release       - Build for release"
	@echo "  make check         - Run all checks (format, lint, test)"
	@echo "  make test          - Run tests"
	@echo "  make lint          - Run linter (clippy)"
	@echo "  make format-check  - Check code formatting"
	@echo "  make format        - Format code"
	@echo "  make clean         - Clean build artifacts"
	@echo "  make dev-deps      - Install development dependencies"
	@echo "  make run           - Run the project"
	@echo "  make help          - Show this help message"
