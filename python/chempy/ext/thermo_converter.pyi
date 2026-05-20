from __future__ import annotations

from typing import Optional

from chempy.thermo import NASAModel, ThermoGAModel, WilhoitModel

def convertGAtoWilhoit(
    GAthermo: ThermoGAModel,
    atoms: int,
    rotors: int,
    linear: bool,
    B0: float = ...,
    constantB: bool = ...,
) -> WilhoitModel: ...
def convertWilhoitToNASA(
    wilhoit: WilhoitModel,
    Tmin: float,
    Tmax: float,
    Tint: float,
    fixedTint: bool = ...,
    weighting: bool = ...,
    continuity: int = ...,
) -> NASAModel: ...
def convertCpToNASA(
    CpObject: object,
    H298: float,
    S298: float,
    fixed: int = ...,
    weighting: int = ...,
    tint: float = ...,
    Tmin: float = ...,
    Tmax: float = ...,
    contCons: int = ...,
) -> NASAModel: ...
