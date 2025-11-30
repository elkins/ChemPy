################################################################################
#
#   Makefile for ChemPy - Modern development tasks
#
################################################################################

.PHONY: help build clean test lint format type-check docs install install-dev

help:
	@echo "ChemPy development tasks:"
	@echo "  make build          - Build Cython extensions"
	@echo "  make clean          - Remove build artifacts"
	@echo "  make test           - Run test suite with pytest"
	@echo "  make test-cov       - Run tests with coverage report"
	@echo "  make lint           - Lint code with flake8"
	@echo "  make format         - Format code with black and isort"
	@echo "  make type-check     - Check types with mypy"
	@echo "  make docs           - Build documentation"
	@echo "  make install        - Install package in development mode"
	@echo "  make install-dev    - Install with development dependencies"
	@echo "  make all            - Run full quality checks"

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
	pytest unittest/

test-cov:
	pytest unittest/ --cov=chempy --cov-report=html --cov-report=term

lint:
	flake8 chempy unittest --max-line-length=100 --extend-ignore=E203,W503

format:
	black chempy unittest --line-length=100
	isort chempy unittest

type-check:
	mypy chempy

docs:
	cd documentation && make html

install:
	pip install -e .

install-dev:
	pip install -e ".[dev,docs]"

all: clean lint type-check test build docs
	@echo "✓ All checks passed!"

