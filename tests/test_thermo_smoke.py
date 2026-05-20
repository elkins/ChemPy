from chempy.thermo import ThermoGAModel


def test_thermo_construct_minimal():
    t = ThermoGAModel(
        Tdata=[300.0, 400.0],
        Cpdata=[29.1, 29.2],
        H298=0.0,
        S298=130.0,
        Tmin=300.0,
        Tmax=400.0,
        comment="smoke",
    )
    assert t is not None
    assert t.H298 == 0.0
