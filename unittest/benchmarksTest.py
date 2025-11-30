import pytest
import sys

from chempy.molecule import Molecule
from chempy.states import StatesModel, Translation, RigidRotor, HarmonicOscillator

# Skip SMILES tests on Windows where OpenBabel may not be available
skip_on_windows = pytest.mark.skipif(
    sys.platform == "win32",
    reason="OpenBabel not available on Windows"
)


@skip_on_windows
@pytest.mark.benchmark(group="molecule")
def test_bench_molecule_from_smiles_benzene(benchmark):
    def build():
        m = Molecule()
        m.fromSMILES("c1ccccc1")
        # Exercise some graph features
        _ = m.getSmallestSetOfSmallestRings()
        _ = m.calculateSymmetryNumber()
        return m
    benchmark(build)


@skip_on_windows
@pytest.mark.benchmark(group="molecule")
def test_bench_molecule_from_smiles_ethane_rotors(benchmark):
    def build():
        m = Molecule(SMILES="CC")
        _ = m.countInternalRotors()
        return m
    benchmark(build)


@pytest.mark.benchmark(group="states")
def test_bench_density_of_states_ilt(benchmark):
    modes = [
        Translation(mass=0.028054),
        RigidRotor(linear=False, inertia=[1e-46, 2e-46, 3e-46], symmetry=1),
        HarmonicOscillator(frequencies=[500.0, 1000.0, 1500.0, 3000.0]),
    ]
    sm = StatesModel(modes=modes, spinMultiplicity=1)

    import numpy as np
    Elist = np.linspace(0.0, 2.0e5, 200)  # 0 to 200 kJ/mol in J/mol

    def run():
        return sm.getDensityOfStatesILT(Elist)

    benchmark(run)


@pytest.mark.benchmark(group="states")
def test_bench_states_construction(benchmark):
    def build_states():
        modes = [
            Translation(mass=0.028054),
            RigidRotor(linear=False, inertia=[1e-46, 2e-46, 3e-46], symmetry=1),
            HarmonicOscillator(frequencies=[500.0, 1000.0, 1500.0, 3000.0]),
        ]
        return StatesModel(modes=modes, spinMultiplicity=1)

    benchmark(build_states)
