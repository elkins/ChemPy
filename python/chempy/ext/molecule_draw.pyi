from __future__ import annotations

from typing import TYPE_CHECKING, Any, Optional, Tuple

if TYPE_CHECKING:
    from chempy.molecule import Molecule

def createNewSurface(
    type: str,
    path: Optional[str] = ...,
    width: int = ...,
    height: int = ...,
) -> Any: ...
def drawMolecule(
    molecule: Molecule,
    path: Optional[str] = ...,
    surface: str = ...,
) -> Tuple[Any, Any, Tuple[int, int, int, int]]: ...
