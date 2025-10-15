# ScortonJS 🛡️

[![CI/CD](https://github.com/scortonjs/scortonjs/workflows/CI/CD/badge.svg)](https://github.com/scortonjs/scortonjs/actions)
[![PyPI version](https://badge.fury.io/py/scortonjs.svg)](https://badge.fury.io/py/scortonjs)
[![Python 3.8+](https://img.shields.io/badge/python-3.8+-blue.svg)](https://www.python.org/downloads/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Code style: black](https://img.shields.io/badge/code%20style-black-000000.svg)](https://github.com/psf/black)

> **Production-ready security auditing framework with comprehensive scanning and compliance tools**

ScortonJS is a powerful, enterprise-grade security framework for auditing, vulnerability scanning, and compliance assessment. Built with Python, Rust, and TypeScript, it provides fast, reliable security analysis for modern applications and infrastructure.

## ✨ Features

### 🔍 **Comprehensive Security Scanning**
- **DNS Enumeration** - Discover subdomains and DNS records
- **Port Scanning** - Identify open ports and services
- **SSL/TLS Analysis** - Certificate validation and security assessment
- **Security Headers** - HTTP security header analysis
- **Vulnerability Scanning** - XSS, directory traversal, and more
- **Cookie Security** - Cookie configuration analysis

### 📊 **Advanced Scoring & Auditing**
- **Cyberscore Calculation** - Overall security score assessment
- **Deep Audit Mode** - Comprehensive security analysis
- **Compliance Frameworks** - DORA and NIS2 compliance checking
- **Risk Assessment** - Detailed risk analysis and recommendations

### 🚀 **Production-Ready Features**
- **Retry Logic** - Automatic retry with exponential backoff
- **Rate Limiting** - Respectful API usage
- **Comprehensive Logging** - Detailed operation logs
- **Error Handling** - Graceful error recovery
- **Type Safety** - Full type hints and validation
- **Cross-Platform** - Works on Windows, macOS, and Linux

## 🚀 Quick Start

### Installation

#### Option 1: PyPI (Recommended)
```bash
pip install scortonjs
```

#### Option 2: From Source
```bash
git clone https://github.com/scortonjs/scortonjs.git
cd scortonjs
pip install -e .
```

#### Option 3: Development Setup
```bash
git clone https://github.com/scortonjs/scortonjs.git
cd scortonjs
python -m venv venv
source venv/bin/activate  # Windows: venv\Scripts\activate
pip install -r requirements.txt
pip install -e .[dev]
```

### Basic Usage

```bash
# Set up authentication
export SCORTON_API_URL=http://localhost:8000
export SCORTON_TOKEN=your-jwt-token

# Run a DNS enumeration scan
scorton scan dns_enum example.com

# Calculate security score
scorton score example.com

# Run deep audit
scorton audit example.com

# Show configuration
scorton config
```

## 📖 Documentation

### Framework Guide
For comprehensive framework documentation, see [FRAMEWORK_GUIDE.md](FRAMEWORK_GUIDE.md)

### Command Reference

#### `scan` - Security Scanning
```bash
# DNS enumeration
scorton scan dns_enum example.com

# Port scanning
scorton scan port_scan 192.168.1.1

# SSL certificate analysis
scorton scan ssl_scan https://example.com

# Security headers check
scorton scan headers_check https://example.com

# Available tools: cookie_scan, dir_scan, dns_enum, headers_check, 
# methods_scan, port_scan, reverse_dns, ssl_scan, url_analyze, 
# whois_scan, xss_scan
```

#### `score` - Security Scoring
```bash
# Calculate overall security score
scorton score example.com

# With custom API endpoint
scorton score example.com --api https://api.scorton.com --token your-token
```

#### `audit` - Deep Security Audit
```bash
# Run comprehensive security audit
scorton audit example.com

# With verbose output
scorton audit example.com --verbose
```

#### `config` - Configuration Management
```bash
# Show current configuration
scorton config

# Get environment variable setup command
scorton config --set SCORTON_API_URL=https://api.scorton.com
```

### Advanced Usage

#### Custom Timeout and Retry Settings
```bash
scorton scan dns_enum example.com --timeout 60
```

#### Verbose Logging
```bash
scorton scan dns_enum example.com --verbose
```

#### Programmatic Usage
```python
from scorton import ScortonClient

# Initialize client
client = ScortonClient(
    api_base="http://localhost:8000",
    token="your-jwt-token",
    timeout=30,
    max_retries=3
)

# Run scan
result = client.call_api("dns_enum", "example.com")
print(result)
```

## 🏗️ Architecture

ScortonJS is built as a comprehensive security framework with multiple language components:

### Multi-Language Stack
- **Python** - Main orchestration layer and business logic
- **Rust** - High-performance security scanning and compliance engine
- **TypeScript/Node.js** - Modern CLI interface and API bindings

### Core Components
- **Python Core** (`scorton.py`) - Main framework orchestration
- **Rust Security Engine** - High-performance scanning components
- **Rust Compliance Engine** - DORA and NIS2 compliance checking
- **Node.js CLI** - Modern command-line interface
- **Plugin System** - Extensible architecture for custom tools

### Framework Structure
```
scorton-js/
├── scorton.py              # Python core framework
├── packages/
│   ├── cli/                # Node.js CLI interface
│   └── rust-core/          # Rust security engine
├── FRAMEWORK_GUIDE.md      # Comprehensive documentation
└── tests/                  # Test suite
```

## 🔧 Configuration

### Environment Variables
```bash
# Required
export SCORTON_TOKEN=your-jwt-token

# Optional
export SCORTON_API_URL=http://localhost:8000
```

### Configuration File
Create `~/.scorton/config.yaml`:
```yaml
api:
  base_url: "http://localhost:8000"
  timeout: 30
  max_retries: 3

logging:
  level: "INFO"
  format: "%(asctime)s - %(name)s - %(levelname)s - %(message)s"

security:
  verify_ssl: true
  user_agent: "ScortonCLI/1.0.0"
```

## 🧪 Testing

### Running Tests
```bash
# Run all tests
pytest tests/ -v

# Run with coverage
pytest tests/ --cov=scorton --cov-report=html

# Run specific test
pytest tests/test_scorton.py::TestScortonClient -v
```

### Test Coverage
- **Unit Tests** - Individual component testing
- **Integration Tests** - End-to-end workflow testing
- **API Tests** - Mock API response testing
- **Error Handling Tests** - Exception and error scenarios

## 🚀 Development

### Prerequisites
- Python 3.8+
- Node.js 18+
- Rust 1.70+
- Git

### Development Setup
```bash
# Clone repository
git clone https://github.com/scortonjs/scortonjs.git
cd scortonjs

# Set up Python environment
python -m venv venv
source venv/bin/activate
pip install -r requirements.txt
pip install -e .[dev]

# Set up pre-commit hooks
pre-commit install

# Set up Node.js dependencies
cd packages/cli
npm install

# Set up Rust dependencies
cd ../rust-core
cargo build
```

### Code Quality
```bash
# Format code
black scorton.py

# Lint code
flake8 scorton.py

# Type checking
mypy scorton.py

# Security scan
bandit -r scorton.py
```

## 📊 Performance

### Benchmarks
- **DNS Enumeration**: ~1000 subdomains/second
- **Port Scanning**: ~1000 ports/second
- **SSL Analysis**: ~100 certificates/second
- **Memory Usage**: <50MB typical

### Optimization Features
- **Concurrent Scanning** - Parallel execution for speed
- **Connection Pooling** - Efficient HTTP connections
- **Caching** - Intelligent result caching
- **Resource Management** - Memory and CPU optimization

## 🔒 Security

### Security Features
- **Input Validation** - Comprehensive input sanitization
- **Rate Limiting** - Prevents API abuse
- **Secure Defaults** - Security-first configuration
- **Audit Logging** - Complete operation audit trail

### Vulnerability Reporting
Report security vulnerabilities to: security@scortonjs.com

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

### Good First Issues
- 🟢 Fix documentation typos
- 🟢 Add test coverage
- 🟢 Improve error messages
- 🟡 Add new security tools
- 🟡 Performance optimizations
- 🔴 Architecture improvements

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Security Community** - For tools and techniques
- **Open Source Projects** - For inspiration and libraries
- **Contributors** - For code, documentation, and feedback

## 📞 Support

- **Documentation**: [docs.scortonjs.com](https://docs.scorton.com)
- **Issues**: [GitHub Issues](https://github.com/scortonjs/scortonjs/issues)
- **Discussions**: [GitHub Discussions](https://github.com/scortonjs/scortonjs/discussions)
- **Email**: team@scorton.tech

---

**Made with ❤️ by the ScortonJS Team**
