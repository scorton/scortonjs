#!/usr/bin/env python3
"""
Setup script for ScortonJS - Production-ready security auditing framework.
"""

from setuptools import setup, find_packages
import os

# Read the README file
def read_readme():
    with open("README.md", "r", encoding="utf-8") as fh:
        return fh.read()

# Read requirements
def read_requirements():
    with open("requirements.txt", "r", encoding="utf-8") as fh:
        return [line.strip() for line in fh if line.strip() and not line.startswith("#")]

setup(
    name="scortonjs",
    version="1.0.0",
    author="ScortonJS Team",
    author_email="team@scortonjs.com",
    description="Production-ready security auditing framework with comprehensive scanning and compliance tools",
    long_description=read_readme(),
    long_description_content_type="text/markdown",
    url="https://github.com/scortonjs/scortonjs",
    project_urls={
        "Bug Reports": "https://github.com/scortonjs/scortonjs/issues",
        "Source": "https://github.com/scortonjs/scortonjs",
        "Documentation": "https://docs.scortonjs.com",
    },
    packages=find_packages(),
    classifiers=[
        "Development Status :: 5 - Production/Stable",
        "Intended Audience :: Developers",
        "Intended Audience :: System Administrators",
        "Intended Audience :: Information Technology",
        "Topic :: Security",
        "Topic :: System :: Networking",
        "Topic :: Internet :: WWW/HTTP",
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Python :: 3.12",
        "Programming Language :: Python :: 3.13",
        "Operating System :: OS Independent",
        "Environment :: Console",
    ],
    python_requires=">=3.8",
    install_requires=[
        "requests>=2.32.0",
        "urllib3>=2.0.0",
    ],
    extras_require={
        "dev": [
            "pytest>=7.0.0",
            "pytest-cov>=4.0.0",
            "black>=23.0.0",
            "flake8>=6.0.0",
            "mypy>=1.0.0",
            "pre-commit>=3.0.0",
        ],
        "rich": [
            "rich>=13.0.0",
        ],
        "all": [
            "pytest>=7.0.0",
            "pytest-cov>=4.0.0",
            "black>=23.0.0",
            "flake8>=6.0.0",
            "mypy>=1.0.0",
            "pre-commit>=3.0.0",
            "rich>=13.0.0",
        ],
    },
    entry_points={
        "console_scripts": [
            "scorton=scorton:main",
        ],
    },
    include_package_data=True,
    zip_safe=False,
    keywords="security, audit, cli, cybersecurity, compliance, dora, nis2, scanning",
    platforms=["any"],
)
