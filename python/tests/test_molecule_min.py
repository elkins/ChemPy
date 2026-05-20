from chempy.molecule import Atom, Bond, Molecule


def test_add_remove_hydrogen():
    mol = Molecule()
    c = Atom("C", 0, 1, 0, 0, "")
    mol.addAtom(c)
    h = Atom("H", 0, 1, 0, 0, "")
    mol.addAtom(h)
    mol.addBond(c, h, Bond("S"))
    assert len(mol.vertices) == 2
    mol.removeAtom(h)
    assert len(mol.vertices) == 1
