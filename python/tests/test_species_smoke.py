from chempy.species import Species


def test_species_basic_fields():
    s = Species("H2")
    assert s is not None
    assert isinstance(s.label, str)
