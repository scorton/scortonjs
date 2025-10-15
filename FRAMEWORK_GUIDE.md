# ScortonJS Framework Guide

## Overview

ScortonJS is a comprehensive security scanning and compliance framework that combines Python, Rust, and Node.js components to provide enterprise-grade security analysis capabilities. The framework is designed with modularity, performance, and extensibility in mind.

## Architecture

ScortonJS follows a multi-language architecture:

- **Python Core**: Main orchestration and business logic
- **Rust Components**: High-performance security scanning and compliance checks
- **Node.js CLI**: Modern command-line interface with TypeScript support

## Installation

### Prerequisites

- Python 3.8+
- Rust 1.70+
- Node.js 18+
- npm or pnpm

### Quick Start

```bash
# Clone the repository
git clone <repository-url>
cd scorton-js

# Install Python dependencies
pip install -r requirements.txt

# Install Rust dependencies
cd packages/rust-core
cargo build --release

# Install Node.js dependencies
cd packages/cli
npm install
npm run build
```

## Core Components

### 1. Python Core (`scorton.py`)

The Python core provides the main orchestration layer and business logic for ScortonJS.

**Key Features:**
- Configuration management
- Plugin system for extensibility
- Output formatting and reporting
- Integration with external security tools

**Usage:**
```python
from scorton import ScortonFramework

# Initialize the framework
scorton = ScortonFramework()

# Run a security scan
results = scorton.scan(target="example.com")

# Generate compliance report
compliance = scorton.compliance_check(target="example.com")
```

### 2. Rust Components (`packages/rust-core/`)

High-performance security scanning components written in Rust.

**Available Crates:**
- `scorton-security`: Core security scanning functionality
- `scorton-compliance`: Compliance checking (DORA, NIS2)
- `scorton-napi`: Node.js API bindings
- `scorton-server`: Web server for API access

**Usage:**
```rust
use scorton_security::Scanner;
use scorton_compliance::DoraChecker;

// Initialize scanner
let scanner = Scanner::new();
let results = scanner.scan("example.com").await?;

// Run compliance check
let dora_checker = DoraChecker::new();
let compliance = dora_checker.check_deployment_frequency(&results)?;
```

### 3. Node.js CLI (`packages/cli/`)

Modern command-line interface built with TypeScript.

**Available Commands:**
- `scan`: Run security scans
- `compliance`: Check compliance standards
- `audit`: Perform security audits
- `score`: Calculate security scores
- `report`: Generate detailed reports

**Usage:**
```bash
# Run a security scan
npx scorton scan example.com

# Check DORA compliance
npx scorton compliance dora example.com

# Generate security score
npx scorton score example.com

# Create detailed report
npx scorton report example.com --format json
```

## Configuration

### Configuration File (`scorton.json`)

ScortonJS uses a JSON configuration file to manage settings:

```json
{
  "target": "example.com",
  "scan_types": ["ssl", "headers", "dns", "security"],
  "output_format": "json",
  "compliance_standards": ["dora", "nis2"],
  "plugins": {
    "custom_scanner": {
      "enabled": true,
      "config": {
        "timeout": 30,
        "retries": 3
      }
    }
  }
}
```

### Environment Variables

```bash
# API Keys
export SCORTON_API_KEY="your-api-key"
export PYPI_API_TOKEN="your-pypi-token"

# Configuration
export SCORTON_CONFIG_PATH="/path/to/scorton.json"
export SCORTON_OUTPUT_DIR="/path/to/output"
```

## Usage Examples

### Basic Security Scan

```bash
# Using Node.js CLI
npx scorton scan example.com

# Using Python directly
python scorton.py scan example.com

# Using Rust components
cargo run --bin scorton-server
```

### Compliance Checking

```bash
# Check DORA compliance
npx scorton compliance dora example.com

# Check NIS2 compliance
npx scorton compliance nis2 example.com

# Check multiple standards
npx scorton compliance --standards dora,nis2 example.com
```

### Advanced Scanning

```bash
# Custom scan with specific modules
npx scorton scan example.com --modules ssl,headers,dns

# Scan with custom configuration
npx scorton scan example.com --config custom-config.json

# Generate multiple output formats
npx scorton scan example.com --format json,markdown,pdf
```

### API Usage

```javascript
// Using the Node.js API
import { ScortonClient } from '@scorton/cli';

const client = new ScortonClient({
  apiKey: process.env.SCORTON_API_KEY
});

const results = await client.scan('example.com');
const compliance = await client.checkCompliance('example.com', 'dora');
```

## Plugin System

ScortonJS supports a plugin system for extending functionality:

### Creating a Plugin

```python
# plugins/custom_scanner.py
from scorton.plugins import BasePlugin

class CustomScannerPlugin(BasePlugin):
    def __init__(self, config):
        super().__init__(config)
        self.name = "custom_scanner"
    
    def scan(self, target):
        # Custom scanning logic
        return {
            "target": target,
            "results": self.perform_scan(target)
        }
    
    def perform_scan(self, target):
        # Implementation here
        pass
```

### Registering a Plugin

```python
from scorton import ScortonFramework
from plugins.custom_scanner import CustomScannerPlugin

scorton = ScortonFramework()
scorton.register_plugin(CustomScannerPlugin)
```

## Output Formats

ScortonJS supports multiple output formats:

- **JSON**: Machine-readable format for integration
- **Markdown**: Human-readable reports
- **PDF**: Professional reports for stakeholders
- **XML**: Legacy format support
- **CSV**: Data analysis format

### Example Output

```json
{
  "scan_id": "scan-1234567890",
  "target": "example.com",
  "timestamp": "2024-01-15T10:30:00Z",
  "results": {
    "ssl": {
      "grade": "A",
      "issues": []
    },
    "headers": {
      "security_headers": 8,
      "missing_headers": ["X-Frame-Options"]
    },
    "dns": {
      "dnssec": true,
      "records": ["A", "AAAA", "MX"]
    }
  },
  "compliance": {
    "dora": {
      "deployment_frequency": "high",
      "lead_time": "low",
      "mttr": "medium",
      "change_failure_rate": "low"
    }
  }
}
```

## Development

### Setting Up Development Environment

```bash
# Install development dependencies
pip install -r requirements-dev.txt

# Install pre-commit hooks
pre-commit install

# Run tests
pytest

# Run linting
flake8 scorton.py
black scorton.py
mypy scorton.py
```

### Building from Source

```bash
# Build Python package
python setup.py build

# Build Rust components
cd packages/rust-core
cargo build --release

# Build Node.js CLI
cd packages/cli
npm run build
```

### Testing

```bash
# Run Python tests
pytest tests/

# Run Rust tests
cd packages/rust-core
cargo test

# Run Node.js tests
cd packages/cli
npm test
```

## CI/CD Integration

ScortonJS includes comprehensive CI/CD workflows:

- **Linting**: Code quality checks
- **Security Scanning**: Automated security analysis
- **Testing**: Multi-language test execution
- **Building**: Package compilation
- **Deployment**: Automated publishing

### GitHub Actions

The framework includes pre-configured GitHub Actions workflows:

- `ci.yml`: Complete CI/CD pipeline
- Multi-language support (Python, Rust, Node.js)
- Automated testing and security scanning
- Package building and publishing

## Troubleshooting

### Common Issues

1. **Rust Build Errors**
   ```bash
   # Update Rust toolchain
   rustup update
   
   # Clean and rebuild
   cargo clean
   cargo build --release
   ```

2. **Python Import Errors**
   ```bash
   # Install dependencies
   pip install -r requirements.txt
   
   # Check Python path
   python -c "import sys; print(sys.path)"
   ```

3. **Node.js Build Issues**
   ```bash
   # Clear npm cache
   npm cache clean --force
   
   # Reinstall dependencies
   rm -rf node_modules package-lock.json
   npm install
   ```

### Debug Mode

Enable debug mode for detailed logging:

```bash
# Set debug environment variable
export SCORTON_DEBUG=1

# Run with verbose output
npx scorton scan example.com --verbose
```

## Contributing

### Code Style

- **Python**: Follow PEP 8, use Black for formatting
- **Rust**: Use rustfmt and clippy
- **TypeScript**: Follow ESLint rules, use Prettier

### Pull Request Process

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and linting
5. Submit a pull request

### Documentation

- Update documentation for new features
- Include usage examples
- Maintain API documentation

## License

ScortonJS is licensed under the MIT License. See `LICENSE` file for details.

## Support

- **Documentation**: [Framework Guide](FRAMEWORK_GUIDE.md)
- **Issues**: GitHub Issues
- **Discussions**: GitHub Discussions
- **Email**: support@scortonjs.com

## Changelog

See `CHANGELOG.md` for version history and updates.
