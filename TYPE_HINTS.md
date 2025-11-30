# Type Hints Guide for ChemPy

This document provides guidelines for adding and maintaining type hints throughout the ChemPy codebase.

## Overview

ChemPy is committed to achieving PEP 561 compliance with comprehensive type hint support. This improves:

- **IDE Support**: Better autocomplete and inline documentation
- **Type Safety**: Early detection of potential bugs
- **Code Documentation**: Types serve as inline documentation
- **Maintainability**: Clearer function contracts

## Status

✅ **Infrastructure**: PEP 561 marker (`py.typed`) is in place
✅ **Core Modules**: Type hints added to foundational modules
🔄 **In Progress**: Adding type hints to remaining modules

## Quick Start

### Importing Type Hints

```python
from __future__ import annotations  # PEP 563 - postponed evaluation

from typing import (
    TYPE_CHECKING,
    List,
    Dict,
    Optional,
    Tuple,
    Union,
    Any,
    Callable,
    Iterable,
)

# Forward references (to avoid circular imports)
if TYPE_CHECKING:
    from chempy.molecule import Molecule
    from chempy.geometry import Geometry
```

### Class Annotations

```python
class Element:
    """A chemical element."""
    
    number: int
    symbol: str
    name: str
    mass: float
    
    def __init__(self, number: int, symbol: str, name: str, mass: float) -> None:
        """Initialize an Element."""
        self.number = number
        self.symbol = symbol
        self.name = name
        self.mass = mass
```

### Method Annotations

```python
def getElement(number: int = 0, symbol: str = '') -> Optional[Element]:
    """
    Get an Element by atomic number or symbol.
    
    Args:
        number: Atomic number (0 to match any).
        symbol: Element symbol ('' to match any).
    
    Returns:
        Element: The matching element, or None if not found.
    
    Raises:
        ChemPyError: If no element matches the criteria.
    """
    ...
```

## Common Patterns

### Collections

```python
# List of Species
species_list: List[Species] = []

# Dictionary mapping symbols to Elements
elements_dict: Dict[str, Element] = {}

# Tuple of floats
coordinates: Tuple[float, float, float] = (0.0, 0.0, 0.0)

# Optional value
geometry: Optional[Geometry] = None

# Union type (when multiple types are possible)
value: Union[int, float] = 3.14
```

### Function Signatures

```python
# Simple function
def calculate(x: float, y: float) -> float:
    """Calculate something."""
    return x + y

# Function with optional arguments
def process(
    data: List[float],
    threshold: float = 1e-6,
    verbose: bool = False,
) -> Tuple[List[float], Dict[str, Any]]:
    """Process data."""
    ...

# Function that accepts any callable
def apply_transform(
    func: Callable[[float], float],
    values: List[float],
) -> List[float]:
    """Apply function to values."""
    return [func(v) for v in values]
```

### Forward References

For circular dependencies, use `TYPE_CHECKING`:

```python
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from chempy.molecule import Molecule

class Reaction:
    molecules: List[Molecule]
    
    def __init__(self, molecules: Optional[List[Molecule]] = None):
        self.molecules = molecules or []
```

### Class Variables

```python
from typing import Final, ClassVar

class Constants:
    """Physical constants."""
    
    # Immutable constant
    NA: Final[float] = 6.02214179e23
    
    # Class variable shared by all instances
    unit_system: ClassVar[str] = "SI"
```

## Module-Specific Guidelines

### chempy/constants.py

- All constants should be annotated with `Final[float]` or `Final[int]`
- Include docstrings with unit information

### chempy/element.py

- Element class fully typed
- Use `List[Element]` for collections

### chempy/species.py

- Use `TYPE_CHECKING` for Molecule, Geometry, etc.
- Ensure `__init__` has complete type signature

### chempy/reaction.py

- Reactants/products: `List[Species]`
- Kinetics model: `Optional[KineticsModel]`

### chempy/molecule.py

- Use forward references for circular deps
- Atom lists: `List[Atom]`
- Bond maps: `Dict[Tuple[int, int], Bond]`

## Mypy Configuration

The project uses mypy for type checking. Configuration is in `pyproject.toml`:

```toml
[tool.mypy]
python_version = "3.8"
warn_return_any = true
warn_unused_configs = true
disallow_untyped_defs = false
ignore_missing_imports = true
```

To run type checking:

```bash
make type-check
# or
mypy chempy/
```

## Best Practices

### 1. Be Specific

```python
# ✅ Good - specific type
def process(items: List[Species]) -> Dict[str, float]:
    ...

# ❌ Avoid - too generic
def process(items):
    ...
```

### 2. Use Optional for Nullable Values

```python
# ✅ Good - explicitly optional
def get_property(name: str) -> Optional[float]:
    ...

# ❌ Unclear - might return None
def get_property(name: str):
    ...
```

### 3. Use Union for Multiple Types

```python
# ✅ Good - both types are valid
def calculate(value: Union[int, float]) -> float:
    ...

# ❌ Avoid - too generic
def calculate(value):
    ...
```

### 4. Document Complex Types

```python
# For complex return types, use docstrings
def analyze(
    molecules: List[Molecule],
    temperature: float,
) -> Tuple[List[Dict[str, Any]], float]:
    """
    Analyze molecules at given temperature.
    
    Returns:
        Tuple of (analysis results list, average energy)
        where each result is a dict with keys: 'id', 'energy', 'stable'
    """
    ...
```

### 5. Gradual Typing

You don't need to type everything at once. It's fine to:

- Start with public APIs
- Add types to frequently-used functions first
- Leave some internal functions untyped initially

```python
# Partially typed is fine
def public_method(self, x: int) -> str:
    # Internal helper without types (for now)
    return self._process(x)

def _process(self, x):  # No types yet
    ...
```

## Adding Type Hints to Existing Code

When adding type hints to existing functions:

1. **Start with the signature**:
   ```python
   def function(param1: Type1, param2: Type2) -> ReturnType:
   ```

2. **Add class attributes**:
   ```python
   class MyClass:
       attr: Type
   ```

3. **Update docstrings** to match the type signature

4. **Run mypy** to check for issues:
   ```bash
   mypy chempy/module.py
   ```

5. **Test** to ensure functionality still works

## Resources

- [PEP 484 - Type Hints](https://www.python.org/dev/peps/pep-0484/)
- [PEP 561 - Distributing Type Information](https://www.python.org/dev/peps/pep-0561/)
- [PEP 563 - Postponed Evaluation of Annotations](https://www.python.org/dev/peps/pep-0563/)
- [Typing Module Documentation](https://docs.python.org/3/library/typing.html)
- [MyPy Documentation](https://mypy.readthedocs.io/)

## Contributing

When contributing code to ChemPy:

1. Add type hints to new functions and classes
2. Use type hints in public APIs
3. Run `make type-check` before submitting
4. Update this guide if adding new patterns

## FAQ

**Q: Should I type all function parameters?**
A: Type public APIs first. Internal/private functions can be typed gradually.

**Q: Can I use `Any`?**
A: Minimize `Any`. Use it only when truly accepting any type, not as a shortcut.

**Q: What if I have circular imports?**
A: Use `TYPE_CHECKING` and forward references as shown above.

**Q: Do I need to type global variables?**
A: Yes, constants and module-level variables should have types.

---

For questions or suggestions, please open an issue on GitHub.
