"""
Cython compatibility module for optional Cython support.

This module provides a graceful fallback for when Cython is not installed.
"""

try:
    import cython
    HAS_CYTHON = True
except ImportError:
    HAS_CYTHON = False
    
    # Provide a dummy cython module for compatibility
    class _DummyCython:
        """Dummy Cython module for when Cython is not installed."""
        pass
    
    cython = _DummyCython()

__all__ = ['cython', 'HAS_CYTHON']
