#!/bin/bash
# ChemPy Quick Start Script
# This script helps set up ChemPy for development

set -e

echo "🔬 ChemPy Development Setup"
echo "============================"
echo ""

# Check Python version
PYTHON_VERSION=$(python3 --version 2>&1 | awk '{print $2}')
echo "✓ Python version: $PYTHON_VERSION"

# Create virtual environment if it doesn't exist
if [ ! -d "venv" ]; then
    echo "📦 Creating virtual environment..."
    python3 -m venv venv
    source venv/bin/activate
else
    echo "✓ Virtual environment already exists"
    source venv/bin/activate
fi

# Upgrade pip
echo "📦 Upgrading pip..."
pip install --upgrade pip setuptools wheel

# Install development dependencies
echo "📦 Installing dependencies..."
pip install -e ".[dev,docs]"

# Build Cython extensions
echo "🔨 Building Cython extensions..."
python setup.py build_ext --inplace

# Run tests
echo "🧪 Running tests..."
pytest unittest/ -v

echo ""
echo "✅ ChemPy is ready for development!"
echo ""
echo "Quick commands:"
echo "  make test       - Run tests"
echo "  make format     - Format code"
echo "  make lint       - Lint code"
echo "  make build      - Build extensions"
echo "  make help       - Show all commands"
echo ""
echo "For more info, see DEVELOPMENT.md"
