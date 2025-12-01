from chempy.kinetics import ArrheniusModel


def test_arrhenius_construct_minimal():
    a = ArrheniusModel(A=1.0, n=0.0, Ea=0.0, T0=1.0)
    assert a is not None
    assert a.A == 1.0


def test_arrhenius_rate_coefficient():
    a = ArrheniusModel(A=2.0, n=0.0, Ea=0.0, T0=1.0)
    k = a.getRateCoefficient(T=300.0)
    assert k == 2.0
