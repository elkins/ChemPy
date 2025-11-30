################################################################################
#
#   Makefile for ChemPy - Modern development tasks
#
################################################################################

.PHONY: help build clean test lint format type-check docs install install-dev check-all structure tox

help:
	@echo "ChemPy development tasks:"
	@echo ""
	@echo "Build & Installation:"
	@echo "  make build          - Build Cython extensions"
	@echo "  make install        - Install package in development mode"
	@echo "  make install-dev    - Install with development dependencies"
	@echo ""
	@echo "Testing:"
	@echo "  make test           - Run test suite (unittest + tests/)"
	@echo "  make test-unit      - Run unit tests only"
	@echo "  make test-cov       - Run tests with coverage report"
	@echo "  make test-fast      - Run tests in parallel"
	@echo "  make tox            - Run tests across Python versions with tox"
	@echo ""
	@echo "Code Quality:"
	@echo "  make lint           - Lint code with flake8"
	@echo "  make format         - Format code with black and isort"
	@echo "  make type-check     - Check types with mypy"
	@echo "  make check          - Run lint, type-check, and test"
	@echo ""
	@echo "Documentation & Info:"
	@echo "  make docs           - Build documentation"
	@echo "  make structure      - Display project structure information"
	@echo ""
	@echo "Maintenance:"
	@echo "  make clean          - Remove build artifacts"
	@echo "  make all            - Run full quality checks and build"

build:
	python setup.py build_ext --inplace

clean:
	python setup.py clean --all
	rm -rf build dist *.egg-info
	find . -type d -name __pycache__ -exec rm -rf {} + 2>/dev/null || true
	find . -type f -name "*.pyc" -delete
	find . -type f -name "*.so" -delete
	find . -type f -name "*.pyd" -delete
	find chempy -type f -name "*.c" -not -name "*_wrapper.c" -delete
	find chempy -type f -name "*.html" -delete
	rm -rf .pytest_cache .coverage htmlcov .mypy_cache .tox

test:
	pytest unittest/ tests/ -v

test-unit:
	pytest unittest/ -v

test-new:
	pytest tests/ -v

test-cov:
	pytest unittest/ tests/ --cov=chempy --cov-report=html --cov-report=term

test-fast:
	pytest unittest/ tests/ -v -n auto

lint:
	flake8 chempy unittest tests --max-line-length=100 --extend-ignore=E203,W503

format:
	black chempy unittest tests --line-length=100
	isort chempy unittest tests

type-check:
	mypy chempy

docs:
	cd documentation && make html

structure:
	@cat STRUCTURE.md

install:
	pip install -e .

install-dev:
	pip install -e ".[dev,docs,test]"

check: lint type-check test
	@echo "✓ All checks passed!"

all: clean check build docs
	@echo "✓ Complete build successful!"

tox:
	tox

