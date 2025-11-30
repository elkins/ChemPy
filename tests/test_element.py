from chempy import element


def test_element_hydrogen_properties():
    h = element.getElement(number=1)
    assert h.symbol == "H"
    # Mass is in kg/mol; hydrogen ~1e-3 kg/mol
    assert h.mass > 1e-3
