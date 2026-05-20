"""
Configuration for benchmark tests.
"""

import sys
from pathlib import Path

# Ensure the parent directory is in the path for imports
benchmark_dir = Path(__file__).parent
project_root = benchmark_dir.parent
if str(project_root) not in sys.path:
    sys.path.insert(0, str(project_root))
