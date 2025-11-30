
import chempy
from __future__ import annotations
from typing import List, Tuple


class GaussianLog:
    filepath: str

    def __init__(self, filepath: str) -> None: ...
    def loadEnergy(self) -> float: ...
    def loadStates(self) -> "chempy.states.StatesModel": ...


def load_from_gaussian_log(filepath: str) -> GaussianLog: ...
