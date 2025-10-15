# Contributing to ScortonJS

Thank you for your interest in contributing to ScortonJS! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Contributing Guidelines](#contributing-guidelines)
- [Development Workflow](#development-workflow)
- [Testing](#testing)
- [Code Style](#code-style)
- [Documentation](#documentation)
- [Release Process](#release-process)
- [Good First Issues](#good-first-issues)

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you agree to uphold this code.

## Getting Started

### Prerequisites

- Python 3.8 or higher
- Node.js 18 or higher (for CLI components)
- Rust 1.70 or higher (for Rust components)
- Git

### Development Setup

1. **Fork and clone the repository**
   ```bash
   git clone https://github.com/your-username/scortonjs.git
   cd scortonjs
   ```

2. **Set up Python environment**
   ```bash
   python3 -m venv venv
   source venv/bin/activate  # On Windows: venv\Scripts\activate
   pip install -r requirements.txt
   pip install -e .[dev]  # Install development dependencies
   ```

3. **Set up pre-commit hooks**
   ```bash
   pre-commit install
   ```

4. **Set up Node.js dependencies**
   ```bash
   cd packages/cli
   npm install
   ```

5. **Set up Rust dependencies**
   ```bash
   cd packages/rust-core
   cargo build
   ```

## Project Structure

```
scorton-js/
├── .github/                 # GitHub workflows and templates
│   └── workflows/
│       └── ci.yml          # CI/CD pipeline
├── packages/
│   ├── cli/                # TypeScript CLI package
│   │   ├── src/
│   │   ├── dist/
│   │   └── package.json
│   └── rust-core/          # Rust core functionality
│       ├── crates/
│       └── Cargo.toml
├── tests/                  # Test suite
│   └── test_scorton.py
├── scorton.py             # Main Python CLI
├── requirements.txt       # Python dependencies
├── setup.py              # Python package setup
├── pyproject.toml        # Python project configuration
├── .pre-commit-config.yaml # Pre-commit hooks
├── .gitignore            # Git ignore rules
└── README.md             # Project documentation
```

## Contributing Guidelines

### Types of Contributions

We welcome several types of contributions:

- **Bug fixes**: Fix issues in existing functionality
- **Feature additions**: Add new security tools or capabilities
- **Documentation improvements**: Enhance docs, examples, or tutorials
- **Performance optimizations**: Improve speed or resource usage
- **Test coverage**: Add tests for existing or new functionality
- **Code quality**: Refactor code for better maintainability

### Before You Start

1. **Check existing issues**: Look for existing issues or discussions
2. **Create an issue**: For significant changes, create an issue first
3. **Discuss**: For major features, discuss the approach in the issue
4. **Fork**: Fork the repository to your GitHub account

## Development Workflow

### 1. Create a Feature Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-number-description
```

### 2. Make Your Changes

- Write clean, well-documented code
- Follow the existing code style
- Add tests for new functionality
- Update documentation as needed

### 3. Test Your Changes

```bash
# Run Python tests
pytest tests/ -v

# Run linting
flake8 scorton.py
black --check scorton.py
mypy scorton.py

# Run Rust tests
cd packages/rust-core
cargo test

# Run Node.js tests
cd packages/cli
npm test
```

### 4. Commit Your Changes

```bash
git add .
git commit -m "feat: add new security scanning tool"
```

**Commit Message Format:**
- `feat:` New features
- `fix:` Bug fixes
- `docs:` Documentation changes
- `style:` Code style changes
- `refactor:` Code refactoring
- `test:` Test additions/changes
- `chore:` Maintenance tasks

### 5. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Then create a pull request on GitHub.

## Testing

### Running Tests

```bash
# Run all tests
pytest tests/ -v

# Run with coverage
pytest tests/ --cov=scorton --cov-report=html

# Run specific test file
pytest tests/test_scorton.py -v

# Run specific test
pytest tests/test_scorton.py::TestScortonClient::test_client_initialization -v
```

### Writing Tests

- Write tests for all new functionality
- Aim for high test coverage (>90%)
- Use descriptive test names
- Test both success and error cases
- Mock external dependencies

### Test Structure

```python
class TestYourFeature:
    """Test cases for your feature."""
    
    def test_success_case(self):
        """Test successful execution."""
        # Arrange
        # Act
        # Assert
    
    def test_error_case(self):
        """Test error handling."""
        # Test error conditions
```

## Code Style

### Python Code Style

- Follow PEP 8 guidelines
- Use Black for formatting (line length: 100)
- Use type hints for all functions
- Write comprehensive docstrings
- Use meaningful variable and function names

### Example Code Style

```python
def scan_target(
    target: str, 
    tool: str, 
    timeout: int = 30
) -> Dict[str, Any]:
    """
    Scan a target using the specified tool.
    
    Args:
        target: The target domain or URL to scan
        tool: The security tool to use
        timeout: Request timeout in seconds
        
    Returns:
        Dictionary containing scan results
        
    Raises:
        APIError: If the scan fails
        ConfigurationError: If configuration is invalid
    """
    # Implementation here
```

### Rust Code Style

- Follow Rust conventions
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Write comprehensive documentation

### TypeScript Code Style

- Use ESLint configuration
- Follow TypeScript best practices
- Use meaningful names
- Add JSDoc comments

## Documentation

### Code Documentation

- Write docstrings for all public functions
- Include type hints
- Document parameters and return values
- Add examples for complex functions

### User Documentation

- Update README.md for user-facing changes
- Add examples for new features
- Document configuration options
- Update installation instructions

### API Documentation

- Document all API endpoints
- Include request/response examples
- Document error codes and messages

## Release Process

### Version Numbering

We follow [Semantic Versioning](https://semver.org/):
- `MAJOR.MINOR.PATCH`
- `1.0.0` for initial release
- Increment based on change type

### Release Steps

1. **Update version numbers**
2. **Update CHANGELOG.md**
3. **Create release branch**
4. **Run full test suite**
5. **Create GitHub release**
6. **Publish to PyPI**

## Good First Issues

Perfect for new contributors! Look for these labels:

### 🟢 **Good First Issue** - Beginner Friendly

- **Fix documentation typos**
  - Update README.md examples
  - Fix docstring formatting
  - Improve error messages

- **Add test coverage**
  - Write tests for existing functions
  - Add integration tests
  - Improve test documentation

- **Code quality improvements**
  - Add type hints to functions
  - Improve variable names
  - Add error handling

### 🟡 **Intermediate** - Some Experience Required

- **Add new security tools**
  - Implement new scanning capabilities
  - Add compliance checks
  - Extend API endpoints

- **Performance optimizations**
  - Improve API response times
  - Optimize memory usage
  - Add caching mechanisms

### 🔴 **Advanced** - Expert Level

- **Architecture improvements**
  - Refactor core components
  - Add plugin system
  - Implement microservices

- **Security enhancements**
  - Add authentication mechanisms
  - Implement rate limiting
  - Add security headers

## Getting Help

- **GitHub Issues**: For bug reports and feature requests
- **Discussions**: For questions and general discussion
- **Discord**: For real-time chat (link in README)
- **Email**: team@scortonjs.com for private matters

## Recognition

Contributors will be recognized in:
- CONTRIBUTORS.md file
- Release notes
- Project documentation
- Social media acknowledgments

Thank you for contributing to Scorton CLI! 🚀
