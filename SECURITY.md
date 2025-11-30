# Security Policy

## Supported Versions

We support the following versions with security updates:

| Version | Supported          |
| ------- | ------------------ |
| 0.2.x   | :white_check_mark: |
| 0.1.x   | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability in ChemPy, please email security considerations to the maintainers privately rather than using the public issue tracker.

Please include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if available)

We will acknowledge receipt and work on a fix with you.

## Security Best Practices

When using ChemPy:

1. Keep your Python version updated
2. Update ChemPy regularly via `pip install --upgrade chempy`
3. Use virtual environments to isolate dependencies
4. Review code that loads untrusted molecular data

## Dependencies

ChemPy depends on:
- **NumPy**: Regularly updated with security patches
- **SciPy**: Regularly updated with security patches

We regularly update dependency versions to include security patches.
