from chempy.reaction import Reaction
from chempy.species import Species


def test_reaction_construct_and_str():
    a = Species(label="A")
    b = Species(label="B")
    c = Species(label="C")
    rxn = Reaction(index=1, reactants=[a, b], products=[c], reversible=True)
    s = str(rxn)
    assert "A" in s and "B" in s and "C" in s
    assert rxn.hasTemplate([a, b], [c]) is True
