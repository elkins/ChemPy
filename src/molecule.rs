use crate::element::Element;
use crate::graph::{Edge, Graph, Vertex};
use std::fmt;

/// An atom.
#[derive(Debug, Clone, PartialEq)]
pub struct Atom {
    pub element: &'static Element,
    pub radical_electrons: i16,
    pub spin_multiplicity: i16,
    pub implicit_hydrogens: i16,
    pub charge: i16,
    pub label: String,
    pub connectivity1: i32,
    pub connectivity2: i32,
    pub connectivity3: i32,
}

impl Atom {
    pub fn new(element: &'static Element) -> Self {
        Atom {
            element,
            radical_electrons: 0,
            spin_multiplicity: 1,
            implicit_hydrogens: 0,
            charge: 0,
            label: String::new(),
            connectivity1: 0,
            connectivity2: 0,
            connectivity3: 0,
        }
    }
}

impl crate::graph::HasConnectivity for Atom {
    fn connectivity1(&self) -> i32 {
        self.connectivity1
    }
    fn set_connectivity1(&mut self, value: i32) {
        self.connectivity1 = value;
    }
    fn connectivity2(&self) -> i32 {
        self.connectivity2
    }
    fn set_connectivity2(&mut self, value: i32) {
        self.connectivity2 = value;
    }
    fn connectivity3(&self) -> i32 {
        self.connectivity3
    }
    fn set_connectivity3(&mut self, value: i32) {
        self.connectivity3 = value;
    }
}

impl Vertex for Atom {
    fn equivalent(&self, other: &Self) -> bool {
        self.element == other.element
            && self.radical_electrons == other.radical_electrons
            && self.spin_multiplicity == other.spin_multiplicity
            && self.implicit_hydrogens == other.implicit_hydrogens
            && self.charge == other.charge
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BondOrder {
    Single,
    Double,
    Triple,
    Benzene,
}

impl fmt::Display for BondOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BondOrder::Single => write!(f, "S"),
            BondOrder::Double => write!(f, "D"),
            BondOrder::Triple => write!(f, "T"),
            BondOrder::Benzene => write!(f, "B"),
        }
    }
}

/// A chemical bond.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bond {
    pub order: BondOrder,
}

impl Bond {
    pub fn new(order: BondOrder) -> Self {
        Bond { order }
    }
}

impl Edge for Bond {
    fn equivalent(&self, other: &Self) -> bool {
        self.order == other.order
    }
}

/// A representation of a molecular structure.
#[derive(Debug, Clone, Default)]
pub struct Molecule {
    pub graph: Graph<Atom, Bond>,
}

impl Molecule {
    pub fn new() -> Self {
        Molecule {
            graph: Graph::new(),
        }
    }

    pub fn add_atom(&mut self, atom: Atom) -> usize {
        self.graph.add_vertex(atom)
    }

    pub fn add_bond(&mut self, v1: usize, v2: usize, bond: Bond) {
        self.graph.add_edge(v1, v2, bond);
    }

    pub fn get_atom(&self, index: usize) -> Option<&Atom> {
        self.graph.vertices.get(index)
    }

    pub fn get_bond(&self, v1: usize, v2: usize) -> Option<&Bond> {
        self.graph.get_edge(v1, v2)
    }

    pub fn to_adjacency_list(&self) -> String {
        let mut result = String::new();
        for (i, atom) in self.graph.vertices.iter().enumerate() {
            let mut line = format!("{} {} {}", i + 1, atom.element.symbol, atom.radical_electrons);
            let mut neighbors: Vec<_> = self.graph.edges[i].keys().collect();
            neighbors.sort();
            for &neighbor in neighbors {
                let bond = self.get_bond(i, neighbor).unwrap();
                line.push_str(&format!(" {{{},{}}}", neighbor + 1, bond.order));
            }
            result.push_str(&line);
            result.push('\n');
        }
        result
    }

    pub fn from_adjacency_list(&mut self, adj_list: &str) {
        use crate::element::get_element;
        self.graph = Graph::new();
        let lines: Vec<&str> = adj_list.trim().lines().collect();
        let mut bond_info = Vec::new();

        for line in &lines {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 3 {
                continue;
            }
            let symbol = parts[1];
            let radical = parts[2].parse::<i16>().unwrap_or(0);
            let element = get_element(0, symbol).expect("Unknown element");
            let mut atom = Atom::new(element);
            atom.radical_electrons = radical;
            self.add_atom(atom);

            // Collect bonds to add after all atoms are created
            for &part in &parts[3..] {
                if part.starts_with('{') && part.ends_with('}') {
                    let inner = &part[1..part.len() - 1];
                    let bond_parts: Vec<&str> = inner.split(',').collect();
                    if bond_parts.len() == 2 {
                        let target_idx = bond_parts[0].parse::<usize>().unwrap() - 1;
                        let order_str = bond_parts[1];
                        let order = match order_str {
                            "S" => BondOrder::Single,
                            "D" => BondOrder::Double,
                            "T" => BondOrder::Triple,
                            "B" => BondOrder::Benzene,
                            _ => BondOrder::Single,
                        };
                        bond_info.push((self.graph.vertices.len() - 1, target_idx, order));
                    }
                }
            }
        }

        for (v1, v2, order) in bond_info {
            if v1 < v2 {
                self.add_bond(v1, v2, Bond::new(order));
            }
        }
    }

    pub fn is_isomorphic(&self, other: &Molecule) -> bool {
        self.graph.is_isomorphic(&other.graph)
    }

    pub fn is_cyclic(&self) -> bool {
        self.graph.is_cyclic()
    }

    pub fn is_linear(&self) -> bool {
        let atom_count = self.graph.vertices.len();

        if atom_count <= 1 {
            return false;
        }
        if atom_count == 2 {
            return true;
        }
        if self.is_cyclic() {
            return false;
        }

        // A molecule is linear if all atoms have degree <= 2
        for adj in &self.graph.edges {
            if adj.len() > 2 {
                return false;
            }
        }

        // Check for specific linear bond patterns:
        // 1. All double bonds (e.g., O=C=O)
        let mut all_double = true;
        for adj in &self.graph.edges {
            for bond in adj.values() {
                if bond.order != BondOrder::Double {
                    all_double = false;
                    break;
                }
            }
        }
        if all_double {
            return true;
        }

        // 2. Alternating single and triple bonds (e.g., H-C#C-H)
        let mut single_triple = true;
        for adj in &self.graph.edges {
            for bond in adj.values() {
                if bond.order != BondOrder::Single && bond.order != BondOrder::Triple {
                    single_triple = false;
                    break;
                }
            }
        }
        if single_triple {
            // Need at least one triple bond for this to be definitely linear in this simplified model
            let mut has_triple = false;
            for adj in &self.graph.edges {
                for bond in adj.values() {
                    if bond.order == BondOrder::Triple {
                        has_triple = true;
                        break;
                    }
                }
            }
            if has_triple {
                return true;
            }
        }

        false
    }

    pub fn is_subgraph_isomorphic(&self, other: &Molecule) -> bool {
        self.graph.is_subgraph_isomorphic(&other.graph)
    }

    pub fn find_subgraph_isomorphisms(&self, other: &Molecule) -> Vec<std::collections::HashMap<usize, usize>> {
        self.graph.find_subgraph_isomorphisms(&other.graph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element;

    #[test]
    fn test_molecule_basic() {
        let mut mol = Molecule::new();
        let c1 = mol.add_atom(Atom::new(&element::C));
        let c2 = mol.add_atom(Atom::new(&element::C));
        mol.add_bond(c1, c2, Bond::new(BondOrder::Single));

        assert_eq!(mol.graph.vertices.len(), 2);
        assert_eq!(mol.get_atom(c1).unwrap().element.symbol, "C");
        assert_eq!(mol.get_bond(c1, c2).unwrap().order, BondOrder::Single);
    }

    #[test]
    fn test_atom_equivalence() {
        let a1 = Atom::new(&element::C);
        let a2 = Atom::new(&element::C);
        let a3 = Atom::new(&element::O);

        assert!(a1.equivalent(&a2));
        assert!(!a1.equivalent(&a3));
    }

    #[test]
    fn test_from_adjacency_list() {
        let adj_list = "
        1 C 0 {2,D}
        2 C 0 {1,D} {3,S}
        3 C 0 {2,S} {4,D}
        4 C 0 {3,D} {5,S}
        5 C 1 {4,S} {6,S}
        6 C 0 {5,S}
        ";
        let mut mol = Molecule::new();
        mol.from_adjacency_list(adj_list);

        assert_eq!(mol.graph.vertices.len(), 6);
        assert_eq!(mol.get_atom(4).unwrap().radical_electrons, 1);
        assert_eq!(mol.get_bond(0, 1).unwrap().order, BondOrder::Double);
    }

    #[test]
    fn test_subgraph_isomorphism() {
        let mut mol = Molecule::new();
        mol.from_adjacency_list("
        1 C 0 {2,D}
        2 C 0 {1,D} {3,S}
        3 C 0 {2,S}
        ");

        let mut pattern = Molecule::new();
        pattern.from_adjacency_list("
        1 C 0 {2,D}
        2 C 0 {1,D}
        ");

        assert!(mol.is_subgraph_isomorphic(&pattern));
        let mappings = mol.find_subgraph_isomorphisms(&pattern);
        assert_eq!(mappings.len(), 2); // C=C can be mapped in 2 ways
    }

    #[test]
    fn test_is_linear() {
        let mut mol = Molecule::new();
        mol.from_adjacency_list("
        1 O 0 {2,D}
        2 O 0 {1,D}
        ");
        assert!(mol.is_linear());

        let mut mol2 = Molecule::new();
        mol2.from_adjacency_list("
        1 O 0 {2,D}
        2 C 0 {1,D} {3,D}
        3 O 0 {2,D}
        ");
        assert!(mol2.is_linear());

        let mut mol3 = Molecule::new();
        mol3.from_adjacency_list("
        1 C 0 {2,S} {3,S}
        2 H 0 {1,S}
        3 H 0 {1,S}
        ");
        assert!(!mol3.is_linear());
    }
}
