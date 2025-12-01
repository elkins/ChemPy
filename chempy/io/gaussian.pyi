from __future__ import annotations

from typing import TYPE_CHECKING, List, Tuple

if TYPE_CHECKING:
    from chempy.states import StatesModel

class GaussianLog:
    filepath: str

    def __init__(self, filepath: str) -> None: ...
    def loadEnergy(self) -> float: ...
    def loadStates(self) -> StatesModel: ...

def load_from_gaussian_log(filepath: str) -> GaussianLog: ...
