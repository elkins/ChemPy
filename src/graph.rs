use std::collections::HashMap;

/// A base trait for vertices in a graph.
pub trait Vertex: Clone {
    fn equivalent(&self, _other: &Self) -> bool {
        true
    }

    fn is_specific_case_of(&self, _other: &Self) -> bool {
        true
    }
}

/// A base trait for edges in a graph.
pub trait Edge: Clone {
    fn equivalent(&self, _other: &Self) -> bool {
        true
    }

    fn is_specific_case_of(&self, _other: &Self) -> bool {
        true
    }
}

pub trait HasConnectivity {
    fn connectivity1(&self) -> i32;
    fn set_connectivity1(&mut self, value: i32);
    fn connectivity2(&self) -> i32;
    fn set_connectivity2(&mut self, value: i32);
    fn connectivity3(&self) -> i32;
    fn set_connectivity3(&mut self, value: i32);
}

/// A simple implementation of a vertex for generic graphs.
#[derive(Debug, Clone, Default)]
pub struct BaseVertex {
    pub connectivity1: i32,
    pub connectivity2: i32,
    pub connectivity3: i32,
    pub sorting_label: i32,
}

impl Vertex for BaseVertex {}

impl HasConnectivity for BaseVertex {
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

/// A simple implementation of an edge for generic graphs.
#[derive(Debug, Clone, Default)]
pub struct BaseEdge {}

impl Edge for BaseEdge {}

/// A graph data type.
#[derive(Debug, Clone)]
pub struct Graph<V: Vertex, E: Edge> {
    pub vertices: Vec<V>,
    pub edges: Vec<HashMap<usize, E>>,
}

impl<V: Vertex, E: Edge> Graph<V, E> {
    pub fn new() -> Self {
        Graph {
            vertices: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: V) -> usize {
        let index = self.vertices.len();
        self.vertices.push(vertex);
        self.edges.push(HashMap::new());
        index
    }

    pub fn add_edge(&mut self, v1: usize, v2: usize, edge: E) {
        self.edges[v1].insert(v2, edge.clone());
        self.edges[v2].insert(v1, edge);
    }

    pub fn get_edge(&self, v1: usize, v2: usize) -> Option<&E> {
        self.edges.get(v1)?.get(&v2)
    }

    pub fn has_edge(&self, v1: usize, v2: usize) -> bool {
        self.edges.get(v1).is_some_and(|adj| adj.contains_key(&v2))
    }

    pub fn remove_vertex(&mut self, index: usize) {
        if index >= self.vertices.len() {
            return;
        }

        // Remove all edges connected to this vertex
        self.edges.remove(index);
        self.vertices.remove(index);

        // Update remaining edges to reflect new indices
        for adj in self.edges.iter_mut() {
            let mut new_adj = HashMap::new();
            for (&neighbor, edge) in adj.iter() {
                if neighbor == index {
                    continue;
                }
                let new_neighbor = if neighbor > index {
                    neighbor - 1
                } else {
                    neighbor
                };
                new_adj.insert(new_neighbor, edge.clone());
            }
            *adj = new_adj;
        }
    }

    pub fn remove_edge(&mut self, v1: usize, v2: usize) {
        if let Some(adj1) = self.edges.get_mut(v1) {
            adj1.remove(&v2);
        }
        if let Some(adj2) = self.edges.get_mut(v2) {
            adj2.remove(&v1);
        }
    }

    pub fn update_connectivity_values(&mut self)
    where
        V: HasConnectivity,
    {
        for i in 0..self.vertices.len() {
            self.vertices[i].set_connectivity1(self.edges[i].len() as i32);
        }

        for i in 0..self.vertices.len() {
            let mut conn2 = 0;
            for &neighbor in self.edges[i].keys() {
                conn2 += self.vertices[neighbor].connectivity1();
            }
            self.vertices[i].set_connectivity2(conn2);
        }

        for i in 0..self.vertices.len() {
            let mut conn3 = 0;
            for &neighbor in self.edges[i].keys() {
                conn3 += self.vertices[neighbor].connectivity2();
            }
            self.vertices[i].set_connectivity3(conn3);
        }
    }

    pub fn is_isomorphic(&self, other: &Graph<V, E>) -> bool {
        self.is_isomorphic_with(other, |v1, v2| v1.equivalent(v2), |e1, e2| e1.equivalent(e2))
    }

    pub fn is_subgraph_isomorphic(&self, other: &Graph<V, E>) -> bool {
        self.is_subgraph_isomorphic_with(other, |v1, v2| v1.equivalent(v2), |e1, e2| e1.equivalent(e2))
    }

    pub fn find_subgraph_isomorphisms(&self, other: &Graph<V, E>) -> Vec<HashMap<usize, usize>> {
        let mut mappings = Vec::new();
        let mut mapping = HashMap::new();
        let mut reverse_mapping = HashMap::new();
        self.vf2_all_matches_with(
            other,
            &mut mapping,
            &mut reverse_mapping,
            0,
            true,
            &mut mappings,
            |v1, v2| v1.equivalent(v2),
            |e1, e2| e1.equivalent(e2),
        );
        mappings
    }

    pub fn is_isomorphic_with<V2: Vertex, E2: Edge>(
        &self,
        other: &Graph<V2, E2>,
        vertex_comparator: impl Fn(&V, &V2) -> bool + Copy,
        edge_comparator: impl Fn(&E, &E2) -> bool + Copy,
    ) -> bool {
        if self.vertices.len() != other.vertices.len() {
            return false;
        }
        if self.vertices.is_empty() {
            return true;
        }

        let mut mapping = HashMap::new();
        let mut reverse_mapping = HashMap::new();
        self.vf2_match_with(
            other,
            &mut mapping,
            &mut reverse_mapping,
            0,
            false,
            vertex_comparator,
            edge_comparator,
        )
    }

    pub fn is_subgraph_isomorphic_with<V2: Vertex, E2: Edge>(
        &self,
        other: &Graph<V2, E2>,
        vertex_comparator: impl Fn(&V, &V2) -> bool + Copy,
        edge_comparator: impl Fn(&E, &E2) -> bool + Copy,
    ) -> bool {
        if self.vertices.len() > other.vertices.len() {
            return false;
        }
        if self.vertices.is_empty() {
            return true;
        }

        let mut mapping = HashMap::new();
        let mut reverse_mapping = HashMap::new();
        // Checks if 'self' is in 'other'
        self.vf2_match_with(
            other,
            &mut mapping,
            &mut reverse_mapping,
            0,
            true,
            vertex_comparator,
            edge_comparator,
        )
    }

    pub fn vf2_match_with<V2: Vertex, E2: Edge>(
        &self,
        other: &Graph<V2, E2>,
        mapping: &mut HashMap<usize, usize>,
        reverse_mapping: &mut HashMap<usize, usize>,
        depth: usize,
        subgraph: bool,
        vertex_comparator: impl Fn(&V, &V2) -> bool + Copy,
        edge_comparator: impl Fn(&E, &E2) -> bool + Copy,
    ) -> bool {
        if depth == self.vertices.len() {
            return true;
        }

        let v1 = depth;
        for v2 in 0..other.vertices.len() {
            if !reverse_mapping.contains_key(&v2)
                && self.is_feasible(
                    v1,
                    v2,
                    other,
                    mapping,
                    subgraph,
                    vertex_comparator,
                    edge_comparator,
                )
            {
                mapping.insert(v1, v2);
                reverse_mapping.insert(v2, v1);

                if self.vf2_match_with(
                    other,
                    mapping,
                    reverse_mapping,
                    depth + 1,
                    subgraph,
                    vertex_comparator,
                    edge_comparator,
                ) {
                    return true;
                }

                mapping.remove(&v1);
                reverse_mapping.remove(&v2);
            }
        }
        false
    }

    pub fn vf2_all_matches_with<V2: Vertex, E2: Edge>(
        &self,
        other: &Graph<V2, E2>,
        mapping: &mut HashMap<usize, usize>,
        reverse_mapping: &mut HashMap<usize, usize>,
        depth: usize,
        subgraph: bool,
        mappings: &mut Vec<HashMap<usize, usize>>,
        vertex_comparator: impl Fn(&V, &V2) -> bool + Copy,
        edge_comparator: impl Fn(&E, &E2) -> bool + Copy,
    ) {
        if depth == self.vertices.len() {
            mappings.push(mapping.clone());
            return;
        }

        let v1 = depth;
        for v2 in 0..other.vertices.len() {
            if !reverse_mapping.contains_key(&v2)
                && self.is_feasible(
                    v1,
                    v2,
                    other,
                    mapping,
                    subgraph,
                    vertex_comparator,
                    edge_comparator,
                )
            {
                mapping.insert(v1, v2);
                reverse_mapping.insert(v2, v1);

                self.vf2_all_matches_with(
                    other,
                    mapping,
                    reverse_mapping,
                    depth + 1,
                    subgraph,
                    mappings,
                    vertex_comparator,
                    edge_comparator,
                );

                mapping.remove(&v1);
                reverse_mapping.remove(&v2);
            }
        }
    }

    fn is_feasible<V2: Vertex, E2: Edge>(
        &self,
        v1: usize,
        v2: usize,
        other: &Graph<V2, E2>,
        mapping: &HashMap<usize, usize>,
        subgraph: bool,
        vertex_comparator: impl Fn(&V, &V2) -> bool,
        edge_comparator: impl Fn(&E, &E2) -> bool,
    ) -> bool {
        // Semantic check
        if !vertex_comparator(&self.vertices[v1], &other.vertices[v2]) {
            return false;
        }

        // Structural check
        for (&neighbor1, edge1) in &self.edges[v1] {
            if let Some(&neighbor2_mapped) = mapping.get(&neighbor1) {
                if let Some(edge2) = other.get_edge(v2, neighbor2_mapped) {
                    if !edge_comparator(edge1, edge2) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        // Degree check
        if subgraph {
            if self.edges[v1].len() > other.edges[v2].len() {
                return false;
            }
        } else if self.edges[v1].len() != other.edges[v2].len() {
            return false;
        }

        true
    }

    pub fn merge(&mut self, other: &Graph<V, E>) {
        let offset = self.vertices.len();
        for vertex in &other.vertices {
            self.add_vertex(vertex.clone());
        }
        for (i, adj) in other.edges.iter().enumerate() {
            for (&neighbor, edge) in adj {
                if i < neighbor {
                    self.add_edge(i + offset, neighbor + offset, edge.clone());
                }
            }
        }
    }

    pub fn split(&self) -> Vec<Graph<V, E>> {
        let mut components = Vec::new();
        let mut visited = vec![false; self.vertices.len()];

        for i in 0..self.vertices.len() {
            if !visited[i] {
                let mut component_indices = Vec::new();
                let mut stack = vec![i];
                visited[i] = true;

                while let Some(u) = stack.pop() {
                    component_indices.push(u);
                    for &v in self.edges[u].keys() {
                        if !visited[v] {
                            visited[v] = true;
                            stack.push(v);
                        }
                    }
                }

                let mut new_graph = Graph::new();
                let mut old_to_new = HashMap::new();
                for &old_idx in &component_indices {
                    let new_idx = new_graph.add_vertex(self.vertices[old_idx].clone());
                    old_to_new.insert(old_idx, new_idx);
                }

                for &old_idx in &component_indices {
                    for (&neighbor, edge) in &self.edges[old_idx] {
                        if old_idx < neighbor {
                            new_graph.add_edge(
                                old_to_new[&old_idx],
                                old_to_new[&neighbor],
                                edge.clone(),
                            );
                        }
                    }
                }
                components.push(new_graph);
            }
        }
        components
    }

    pub fn is_cyclic(&self) -> bool {
        self.has_cycle()
    }

    pub fn has_cycle(&self) -> bool {
        let mut visited = vec![false; self.vertices.len()];
        for i in 0..self.vertices.len() {
            if !visited[i] && self.has_cycle_from(i, None, &mut visited) {
                return true;
            }
        }
        false
    }

    fn has_cycle_from(&self, u: usize, parent: Option<usize>, visited: &mut Vec<bool>) -> bool {
        visited[u] = true;
        for &v in self.edges[u].keys() {
            if Some(v) == parent {
                continue;
            }
            if visited[v] || self.has_cycle_from(v, Some(u), visited) {
                return true;
            }
        }
        false
    }

    pub fn is_vertex_in_cycle(&self, u: usize) -> bool {
        // A vertex is in a cycle if it can reach itself without using the same edge twice
        for &v in self.edges[u].keys() {
            if self.can_reach(v, u, Some(u)) {
                return true;
            }
        }
        false
    }

    fn can_reach(&self, start: usize, target: usize, forbidden_parent: Option<usize>) -> bool {
        let mut visited = vec![false; self.vertices.len()];
        if let Some(p) = forbidden_parent {
            visited[p] = true;
        }
        let mut stack = vec![start];
        visited[start] = true;

        while let Some(u) = stack.pop() {
            if u == target {
                return true;
            }
            for &v in self.edges[u].keys() {
                if !visited[v] {
                    visited[v] = true;
                    stack.push(v);
                }
            }
        }
        false
    }
}

impl<V: Vertex, E: Edge> Default for Graph<V, E> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct TestVertex {
        label: i32,
    }
    impl Vertex for TestVertex {
        fn equivalent(&self, other: &Self) -> bool {
            self.label == other.label
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    struct TestEdge {
        order: i32,
    }
    impl Edge for TestEdge {
        fn equivalent(&self, other: &Self) -> bool {
            self.order == other.order
        }
    }

    #[test]
    fn test_graph_basic() {
        let mut g: Graph<TestVertex, TestEdge> = Graph::new();
        let v1 = g.add_vertex(TestVertex { label: 1 });
        let v2 = g.add_vertex(TestVertex { label: 2 });
        g.add_edge(v1, v2, TestEdge { order: 1 });

        assert_eq!(g.vertices.len(), 2);
        assert!(g.has_edge(v1, v2));
        assert!(g.has_edge(v2, v1));
    }

    #[test]
    fn test_remove_vertex() {
        let mut g: Graph<TestVertex, TestEdge> = Graph::new();
        let v1 = g.add_vertex(TestVertex { label: 1 });
        let v2 = g.add_vertex(TestVertex { label: 2 });
        let v3 = g.add_vertex(TestVertex { label: 3 });
        g.add_edge(v1, v2, TestEdge { order: 1 });
        g.add_edge(v2, v3, TestEdge { order: 2 });

        g.remove_vertex(v2);
        assert_eq!(g.vertices.len(), 2);
        assert!(!g.has_edge(0, 1));
    }

    #[test]
    fn test_isomorphism() {
        let mut g1: Graph<TestVertex, TestEdge> = Graph::new();
        let v1 = g1.add_vertex(TestVertex { label: 1 });
        let v2 = g1.add_vertex(TestVertex { label: 2 });
        g1.add_edge(v1, v2, TestEdge { order: 1 });

        let mut g2: Graph<TestVertex, TestEdge> = Graph::new();
        let u1 = g2.add_vertex(TestVertex { label: 1 });
        let u2 = g2.add_vertex(TestVertex { label: 2 });
        g2.add_edge(u1, u2, TestEdge { order: 1 });

        assert!(g1.is_isomorphic(&g2));

        let mut g3: Graph<TestVertex, TestEdge> = Graph::new();
        let w1 = g3.add_vertex(TestVertex { label: 1 });
        let w2 = g3.add_vertex(TestVertex { label: 3 });
        g3.add_edge(w1, w2, TestEdge { order: 1 });

        assert!(!g1.is_isomorphic(&g3));
    }

    #[test]
    fn test_subgraph_isomorphism() {
        let mut g1: Graph<TestVertex, TestEdge> = Graph::new();
        let v1 = g1.add_vertex(TestVertex { label: 1 });
        let v2 = g1.add_vertex(TestVertex { label: 2 });
        g1.add_edge(v1, v2, TestEdge { order: 1 });

        let mut g2: Graph<TestVertex, TestEdge> = Graph::new();
        let u1 = g2.add_vertex(TestVertex { label: 1 });
        let u2 = g2.add_vertex(TestVertex { label: 2 });
        let u3 = g2.add_vertex(TestVertex { label: 3 });
        g2.add_edge(u1, u2, TestEdge { order: 1 });
        g2.add_edge(u2, u3, TestEdge { order: 2 });

        assert!(g1.is_subgraph_isomorphic(&g2));
    }

    #[test]
    fn test_merge() {
        let mut g1: Graph<TestVertex, TestEdge> = Graph::new();
        g1.add_vertex(TestVertex { label: 1 });
        let mut g2: Graph<TestVertex, TestEdge> = Graph::new();
        g2.add_vertex(TestVertex { label: 2 });

        g1.merge(&g2);
        assert_eq!(g1.vertices.len(), 2);
    }

    #[test]
    fn test_split() {
        let mut g: Graph<TestVertex, TestEdge> = Graph::new();
        let v1 = g.add_vertex(TestVertex { label: 1 });
        let v2 = g.add_vertex(TestVertex { label: 2 });
        let v3 = g.add_vertex(TestVertex { label: 3 });
        g.add_edge(v1, v2, TestEdge { order: 1 });

        let components = g.split();
        assert_eq!(components.len(), 2);
    }

    #[test]
    fn test_connectivity_values() {
        #[derive(Debug, Clone, Default)]
        struct ConnVertex {
            c1: i32,
            c2: i32,
            c3: i32,
        }
        impl Vertex for ConnVertex {}
        impl HasConnectivity for ConnVertex {
            fn connectivity1(&self) -> i32 {
                self.c1
            }
            fn set_connectivity1(&mut self, v: i32) {
                self.c1 = v;
            }
            fn connectivity2(&self) -> i32 {
                self.c2
            }
            fn set_connectivity2(&mut self, v: i32) {
                self.c2 = v;
            }
            fn connectivity3(&self) -> i32 {
                self.c3
            }
            fn set_connectivity3(&mut self, v: i32) {
                self.c3 = v;
            }
        }

        let mut g: Graph<ConnVertex, BaseEdge> = Graph::new();
        let v1 = g.add_vertex(ConnVertex::default());
        let v2 = g.add_vertex(ConnVertex::default());
        let v3 = g.add_vertex(ConnVertex::default());
        g.add_edge(v1, v2, BaseEdge::default());
        g.add_edge(v2, v3, BaseEdge::default());

        g.update_connectivity_values();
        assert_eq!(g.vertices[v1].c1, 1);
        assert_eq!(g.vertices[v2].c1, 2);
        assert_eq!(g.vertices[v1].c2, 2);
        assert_eq!(g.vertices[v2].c2, 2);
    }
}
