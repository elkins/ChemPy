from __future__ import annotations

from typing import Any, Optional, Tuple

import chempy

def createNewSurface(
    type: str,
    path: Optional[str] = ...,
    width: int = ...,
    height: int = ...,
) -> Any: ...
def drawMolecule(
    molecule: "chempy.molecule.Molecule",
    path: Optional[str] = ...,
    surface: str = ...,
) -> Tuple[Any, Any, Tuple[int, int, int, int]]: ...
