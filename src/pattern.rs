use crate::graph::{Edge, Graph, Vertex};
use crate::molecule::{Atom, Bond, BondOrder, Molecule};

/// An atom pattern.
#[derive(Debug, Clone, PartialEq)]
pub struct AtomPattern {
    pub atom_type: Vec<String>,
    pub radical_electrons: Vec<i16>,
    pub spin_multiplicity: Vec<i16>,
    pub charge: Vec<i16>,
    pub label: String,
}

impl AtomPattern {
    pub fn new() -> Self {
        AtomPattern {
            atom_type: Vec::new(),
            radical_electrons: Vec::new(),
            spin_multiplicity: Vec::new(),
            charge: Vec::new(),
            label: String::new(),
        }
    }

    pub fn matches(&self, atom: &Atom) -> bool {
        // Match atom type
        if !self.atom_type.is_empty() {
            let mut type_match = false;
            for t in &self.atom_type {
                if t == "R" {
                    type_match = true;
                    break;
                }
                if t == "R!H" && atom.element.symbol != "H" {
                    type_match = true;
                    break;
                }
                if t == atom.element.symbol {
                    type_match = true;
                    break;
                }
            }
            if !type_match {
                return false;
            }
        }

        // Match radical electrons
        if !self.radical_electrons.is_empty() && !self.radical_electrons.contains(&atom.radical_electrons) {
            return false;
        }

        // Match spin multiplicity
        if !self.spin_multiplicity.is_empty() && !self.spin_multiplicity.contains(&atom.spin_multiplicity) {
            return false;
        }

        // Match charge
        if !self.charge.is_empty() && !self.charge.contains(&atom.charge) {
            return false;
        }

        true
    }
}

impl Vertex for AtomPattern {
    fn equivalent(&self, other: &Self) -> bool {
        self.atom_type == other.atom_type
            && self.radical_electrons == other.radical_electrons
            && self.spin_multiplicity == other.spin_multiplicity
            && self.charge == other.charge
    }
}

/// A bond pattern.
#[derive(Debug, Clone, PartialEq)]
pub struct BondPattern {
    pub order: Vec<BondOrder>,
}

impl BondPattern {
    pub fn new(order: Vec<BondOrder>) -> Self {
        BondPattern { order }
    }

    pub fn matches(&self, bond: &Bond) -> bool {
        if self.order.is_empty() {
            return true;
        }
        self.order.contains(&bond.order)
    }
}

impl Edge for BondPattern {
    fn equivalent(&self, other: &Self) -> bool {
        self.order == other.order
    }
}

/// A molecular pattern.
pub struct MoleculePattern {
    pub graph: Graph<AtomPattern, BondPattern>,
}

impl MoleculePattern {
    pub fn new() -> Self {
        MoleculePattern {
            graph: Graph::new(),
        }
    }

    pub fn is_subgraph_isomorphic(&self, molecule: &Molecule) -> bool {
        self.graph.is_subgraph_isomorphic_with(
            &molecule.graph,
            |ap, a| ap.matches(a),
            |bp, b| bp.matches(b),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element;

    #[test]
    fn test_atom_pattern_matching() {
        let mut ap = AtomPattern::new();
        ap.atom_type.push("C".to_string());
        
        let atom_c = Atom::new(element::get_element(0, "C").unwrap());
        let atom_o = Atom::new(element::get_element(0, "O").unwrap());
        
        assert!(ap.matches(&atom_c));
        assert!(!ap.matches(&atom_o));

        let mut ap_r = AtomPattern::new();
        ap_r.atom_type.push("R!H".to_string());
        assert!(ap_r.matches(&atom_c));
        assert!(ap_r.matches(&atom_o));
        
        let atom_h = Atom::new(element::get_element(0, "H").unwrap());
        assert!(!ap_r.matches(&atom_h));
    }

    #[test]
    fn test_molecule_pattern_isomorphism() {
        let mut molecule = Molecule::new();
        let c1 = molecule.add_atom(Atom::new(element::get_element(0, "C").unwrap()));
        let c2 = molecule.add_atom(Atom::new(element::get_element(0, "C").unwrap()));
        molecule.add_bond(c1, c2, Bond::new(BondOrder::Single));

        let mut pattern = MoleculePattern::new();
        let mut ap = AtomPattern::new();
        ap.atom_type.push("C".to_string());
        let p1 = pattern.graph.add_vertex(ap.clone());
        let p2 = pattern.graph.add_vertex(ap);
        pattern.graph.add_edge(p1, p2, BondPattern::new(vec![BondOrder::Single]));

        assert!(pattern.is_subgraph_isomorphic(&molecule));

        let mut pattern_o = MoleculePattern::new();
        let mut ap_o = AtomPattern::new();
        ap_o.atom_type.push("O".to_string());
        pattern_o.graph.add_vertex(ap_o);
        
        assert!(!pattern_o.is_subgraph_isomorphic(&molecule));
    }
}
