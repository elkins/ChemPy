#!/usr/bin/python
# -*- coding: utf-8 -*-

import unittest
from chempy.pattern import AtomPattern, BondPattern, MoleculePattern, atomTypes
from chempy.molecule import Molecule

class PatternTest(unittest.TestCase):
    """
    Contains unit tests for the chempy.pattern module.
    """

    def testAtomPatternEquivalence(self):
        """
        Test the equivalence of atom patterns.
        """
        ap1 = AtomPattern(atomType=['C'], radicalElectrons=[0], spinMultiplicity=[1])
        ap2 = AtomPattern(atomType=['Cs'], radicalElectrons=[0], spinMultiplicity=[1])
        ap3 = AtomPattern(atomType=['Cd'], radicalElectrons=[0], spinMultiplicity=[1])
        ap_r = AtomPattern(atomType=['R'], radicalElectrons=[0], spinMultiplicity=[1])
        ap_rh = AtomPattern(atomType=['R!H'], radicalElectrons=[0], spinMultiplicity=[1])
        
        self.assertTrue(ap1.equivalent(ap2)) # C is equivalent to Cs (C includes Cs)
        self.assertTrue(ap2.equivalent(ap1))
        self.assertTrue(ap1.equivalent(ap3)) # C is equivalent to Cd
        self.assertTrue(ap_r.equivalent(ap1)) # R is equivalent to C
        self.assertTrue(ap_rh.equivalent(ap1)) # R!H is equivalent to C
        self.assertFalse(ap2.equivalent(ap3)) # Cs is NOT equivalent to Cd

    def testAtomPatternSpecificCase(self):
        """
        Test the isSpecificCaseOf method of atom patterns.
        """
        ap_c = AtomPattern(atomType=['C'], radicalElectrons=[0], spinMultiplicity=[1])
        ap_cs = AtomPattern(atomType=['Cs'], radicalElectrons=[0], spinMultiplicity=[1])
        ap_r = AtomPattern(atomType=['R'], radicalElectrons=[0], spinMultiplicity=[1])
        
        self.assertTrue(ap_cs.isSpecificCaseOf(ap_c))
        self.assertTrue(ap_cs.isSpecificCaseOf(ap_r))
        self.assertTrue(ap_c.isSpecificCaseOf(ap_r))
        self.assertFalse(ap_c.isSpecificCaseOf(ap_cs))
        self.assertFalse(ap_r.isSpecificCaseOf(ap_c))

    def testBondPatternEquivalence(self):
        """
        Test the equivalence of bond patterns.
        """
        bp1 = BondPattern(order=['S'])
        bp2 = BondPattern(order=['S', 'D'])
        bp3 = BondPattern(order=['D', 'S'])
        bp4 = BondPattern(order=['D'])
        
        self.assertTrue(bp1.equivalent(bp1))
        self.assertTrue(bp2.equivalent(bp3))
        self.assertFalse(bp1.equivalent(bp2))
        self.assertFalse(bp1.equivalent(bp4))

    def testBondPatternSpecificCase(self):
        """
        Test the isSpecificCaseOf method of bond patterns.
        """
        bp1 = BondPattern(order=['S'])
        bp2 = BondPattern(order=['S', 'D'])
        
        self.assertTrue(bp1.isSpecificCaseOf(bp2))
        self.assertFalse(bp2.isSpecificCaseOf(bp1))

    def testMoleculePatternAdjacencyList(self):
        """
        Test the fromAdjacencyList and toAdjacencyList methods of MoleculePattern.
        """
        adjlist = (
            "label\n"
            "1 * C 0 {2,S} {3,D}\n"
            "2   H 0 {1,S}\n"
            "3   O 0 {1,D}\n"
        )
        pattern = MoleculePattern().fromAdjacencyList(adjlist)
        self.assertEqual(len(pattern.atoms), 3)
        self.assertEqual(len(pattern.bonds), 3) # 1-2, 1-3 (Wait, bonds is a dict of dicts, so 1-2, 2-1, 1-3, 3-1... but len(pattern.edges) should be 2 for undirected?)
        # MoleculePattern inherits from Graph. Graph.edges is Dict[Vertex, Dict[Vertex, Edge]]
        # Let's check how many total edges are stored.
        edge_count = sum(len(v) for v in pattern.edges.values()) // 2
        self.assertEqual(edge_count, 2)
        
        new_adjlist = pattern.toAdjacencyList(label="Test")
        self.assertIn("C", new_adjlist)
        self.assertIn("H", new_adjlist)
        self.assertIn("O", new_adjlist)

    def testWildcardMatching(self):
        """
        Test matching with wildcard atom types.
        """
        molecule = Molecule().fromSMILES("CC") # Ethane
        pattern = MoleculePattern().fromAdjacencyList(
            "1 R!H 0 {2,S}\n"
            "2 R!H 0 {1,S}\n"
        )
        # We need to make sure the molecule has the right info for subgraph isomorphism
        # Molecule.isSubgraphIsomorphic(pattern)
        self.assertTrue(molecule.isSubgraphIsomorphic(pattern))
        
        pattern_h = MoleculePattern().fromAdjacencyList(
            "1 R 0 {2,S}\n"
            "2 H 0 {1,S}\n"
        )
        molecule.makeHydrogensExplicit()
        self.assertTrue(molecule.isSubgraphIsomorphic(pattern_h))

    def testMultipleAtomTypes(self):
        """
        Test matching with multiple atom types in a single AtomPattern.
        """
        molecule_c = Molecule().fromSMILES("C")
        molecule_o = Molecule().fromSMILES("O")
        
        pattern = MoleculePattern().fromAdjacencyList(
            "1 {C,O} 0\n"
        )
        
        self.assertTrue(molecule_c.isSubgraphIsomorphic(pattern))
        self.assertTrue(molecule_o.isSubgraphIsomorphic(pattern))

if __name__ == '__main__':
    unittest.main(testRunner=unittest.TextTestRunner(verbosity=2))
