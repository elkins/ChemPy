from chempy.reaction import Reaction
from chempy.species import Species, TransitionState
from chempy.states import StatesModel


def test_tst_rate_coefficient_minimal():
    # Minimal states with no modes triggers active K-rotor path
    states_react = StatesModel(modes=[], spinMultiplicity=1)
    states_ts = StatesModel(modes=[], spinMultiplicity=1)

    a = Species(label="A", states=states_react, E0=0.0)
    b = Species(label="B", states=states_react, E0=0.0)
    c = Species(label="C", states=states_react, E0=0.0)

    ts = TransitionState(label="TS", states=states_ts, E0=1000.0, frequency=-500.0, degeneracy=1)

    rxn = Reaction(index=1, reactants=[a, b], products=[c], reversible=True, transitionState=ts)

    k = rxn.calculateTSTRateCoefficient(T=300.0)
    assert k > 0.0
