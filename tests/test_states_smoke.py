from chempy.states import HarmonicOscillator, RigidRotor, StatesModel, Translation


def test_states_basic_partition_and_heat_capacity():
    modes = [
        Translation(mass=0.018),  # ~ water molar mass in kg/mol
        RigidRotor(linear=False, inertia=[1e-46, 1.2e-46, 0.9e-46], symmetry=2),
        HarmonicOscillator(frequencies=[500.0, 1000.0, 1500.0]),
    ]
    sm = StatesModel(modes=modes, spinMultiplicity=1)
    Q = sm.getPartitionFunction(300.0)
    Cp = sm.getHeatCapacity(300.0)
    assert Q > 0.0
    assert Cp > 0.0
