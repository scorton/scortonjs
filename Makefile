# ScortonJS - Development Makefile

.PHONY: help install install-dev test lint format typecheck security clean build publish docs

# Default target
help: ## Show this help message
	@echo "ScortonJS - Development Commands"
	@echo "=================================="
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

# Installation
install: ## Install production dependencies
	pip install -r requirements.txt
	pip install -e .

install-dev: ## Install development dependencies
	pip install -r requirements.txt
	pip install -e .[dev]
	pre-commit install
	cd packages/cli && npm install
	cd packages/rust-core && cargo build

# Testing
test: ## Run all tests
	pytest tests/ -v --cov=scorton --cov-report=term-missing --cov-report=html

test-unit: ## Run unit tests only
	pytest tests/test_scorton.py -v

test-integration: ## Run integration tests
	pytest tests/ -v -m integration

test-coverage: ## Run tests with coverage report
	pytest tests/ --cov=scorton --cov-report=html --cov-report=xml

# Code Quality
lint: ## Run linting checks
	flake8 scorton.py --max-line-length=100 --ignore=E203,W503
	cd packages/cli && npm run lint || echo "No lint script found"

format: ## Format code
	black scorton.py --line-length=100
	cd packages/cli && npm run format || echo "No format script found"

format-check: ## Check code formatting
	black --check scorton.py --line-length=100

typecheck: ## Run type checking
	mypy scorton.py --strict --ignore-missing-imports

security: ## Run security scans
	bandit -r scorton.py -f json -o bandit-report.json
	safety check --json --output safety-report.json

# Rust specific
rust-test: ## Run Rust tests
	cd packages/rust-core && cargo test

rust-build: ## Build Rust components
	cd packages/rust-core && cargo build --release

rust-lint: ## Run Rust linting
	cd packages/rust-core && cargo clippy -- -D warnings

rust-format: ## Format Rust code
	cd packages/rust-core && cargo fmt

rust-format-check: ## Check Rust formatting
	cd packages/rust-core && cargo fmt -- --check

# Node.js specific
node-test: ## Run Node.js tests
	cd packages/cli && npm test

node-build: ## Build Node.js components
	cd packages/cli && npm run build

node-lint: ## Run Node.js linting
	cd packages/cli && npm run lint

# Pre-commit
pre-commit: ## Run pre-commit hooks
	pre-commit run --all-files

pre-commit-install: ## Install pre-commit hooks
	pre-commit install

# Build and Package
build: ## Build all components
	python -m build
	cd packages/cli && npm run build
	cd packages/rust-core && cargo build --release

build-python: ## Build Python package
	python -m build

build-check: ## Check built package
	twine check dist/*

# Publishing
publish-test: ## Publish to TestPyPI
	twine upload --repository testpypi dist/*

publish: ## Publish to PyPI
	twine upload dist/*

# Documentation
docs: ## Generate documentation
	@echo "Generating documentation..."
	@echo "Documentation is maintained in README.md and CONTRIBUTING.md"

docs-serve: ## Serve documentation locally
	@echo "No documentation server configured yet"

# Development
dev-setup: ## Complete development setup
	@echo "Setting up development environment..."
	python -m venv venv
	@echo "Activate virtual environment: source venv/bin/activate"
	@echo "Then run: make install-dev"

dev-test: ## Run development tests
	pytest tests/ -v --tb=short

dev-lint: ## Run all linting
	make lint
	make rust-lint
	make node-lint

dev-format: ## Format all code
	make format
	make rust-format
	make node-build

# CI/CD
ci-test: ## Run CI test suite
	pytest tests/ -v --cov=scorton --cov-report=xml
	cd packages/rust-core && cargo test
	cd packages/cli && npm test

ci-lint: ## Run CI linting
	flake8 scorton.py --max-line-length=100 --ignore=E203,W503
	black --check scorton.py --line-length=100
	mypy scorton.py --strict --ignore-missing-imports
	cd packages/rust-core && cargo fmt -- --check
	cd packages/rust-core && cargo clippy -- -D warnings

ci-security: ## Run CI security checks
	bandit -r scorton.py -f json
	safety check --json

# Cleanup
clean: ## Clean build artifacts
	rm -rf build/
	rm -rf dist/
	rm -rf *.egg-info/
	rm -rf .pytest_cache/
	rm -rf .coverage
	rm -rf htmlcov/
	rm -rf coverage.xml
	rm -rf bandit-report.json
	rm -rf safety-report.json
	cd packages/rust-core && cargo clean
	cd packages/cli && rm -rf node_modules/ dist/

clean-all: clean ## Clean everything including virtual environment
	rm -rf venv/
	rm -rf __pycache__/
	find . -type d -name "__pycache__" -exec rm -rf {} +
	find . -type f -name "*.pyc" -delete

# Quick commands
quick-test: ## Quick test run
	pytest tests/test_scorton.py::TestScortonClient::test_client_initialization -v

quick-lint: ## Quick lint check
	flake8 scorton.py --max-line-length=100 --ignore=E203,W503

quick-format: ## Quick format check
	black --check scorton.py --line-length=100

# Version management
version: ## Show current version
	@python -c "import scorton; print(scorton.__version__)"

version-bump: ## Bump version (requires VERSION=patch|minor|major)
	@if [ -z "$(VERSION)" ]; then echo "Usage: make version-bump VERSION=patch|minor|major"; exit 1; fi
	bump2version $(VERSION)

# Docker (if needed)
docker-build: ## Build Docker image
	docker build -t scortonjs .

docker-test: ## Run tests in Docker
	docker run --rm scortonjs pytest tests/ -v

# Help for contributors
contributor-help: ## Show contributor help
	@echo "Contributor Quick Start:"
	@echo "1. make dev-setup"
	@echo "2. source venv/bin/activate"
	@echo "3. make install-dev"
	@echo "4. make test"
	@echo "5. make lint"
	@echo "6. make format"
	@echo ""
	@echo "Before committing:"
	@echo "make pre-commit"
	@echo ""
	@echo "For more details, see CONTRIBUTING.md"
