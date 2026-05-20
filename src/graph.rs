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
            let mut cv2 = 0;
            for &neighbor in self.edges[i].keys() {
                cv2 += self.vertices[neighbor].connectivity1();
            }
            self.vertices[i].set_connectivity2(cv2);
        }

        for i in 0..self.vertices.len() {
            let mut cv3 = 0;
            for &neighbor in self.edges[i].keys() {
                cv3 += self.vertices[neighbor].connectivity2();
            }
            self.vertices[i].set_connectivity3(cv3);
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

                // Sort indices to maintain order and help with mapping
                component_indices.sort();
                let mut new_graph = Graph::new();
                let mut old_to_new = HashMap::new();

                for &old_idx in &component_indices {
                    let new_idx = new_graph.add_vertex(self.vertices[old_idx].clone());
                    old_to_new.insert(old_idx, new_idx);
                }

                for &old_u in &component_indices {
                    for (&old_v, edge) in &self.edges[old_u] {
                        if old_u < old_v {
                            new_graph.add_edge(
                                *old_to_new.get(&old_u).unwrap(),
                                *old_to_new.get(&old_v).unwrap(),
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

    pub fn merge(&self, other: &Graph<V, E>) -> Graph<V, E> {
        let mut new_graph = self.clone();
        let mut old_to_new = HashMap::new();

        for vertex in &other.vertices {
            let new_idx = new_graph.add_vertex(vertex.clone());
            old_to_new.insert(old_to_new.len(), new_idx);
        }

        for (u_idx, adj) in other.edges.iter().enumerate() {
            for (&v_idx, edge) in adj {
                if u_idx < v_idx {
                    new_graph.add_edge(
                        *old_to_new.get(&u_idx).unwrap(),
                        *old_to_new.get(&v_idx).unwrap(),
                        edge.clone(),
                    );
                }
            }
        }
        new_graph
    }

    pub fn is_cyclic(&self) -> bool {
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

    pub fn is_isomorphic(&self, other: &Graph<V, E>) -> bool {
        if self.vertices.len() != other.vertices.len() {
            return false;
        }
        if self.vertices.is_empty() {
            return true;
        }

        let mut mapping = HashMap::new();
        let mut reverse_mapping = HashMap::new();
        self.vf2_match(other, &mut mapping, &mut reverse_mapping, 0, false)
    }

    pub fn is_subgraph_isomorphic(&self, other: &Graph<V, E>) -> bool {
        if self.vertices.len() < other.vertices.len() {
            return false;
        }
        if other.vertices.is_empty() {
            return true;
        }

        let mut mapping = HashMap::new();
        let mut reverse_mapping = HashMap::new();
        // VF2 for subgraph: swap self and other?
        // Actually, Python's self.isSubgraphIsomorphic(other) checks if 'other' is in 'self'
        other.vf2_match(self, &mut mapping, &mut reverse_mapping, 0, true)
    }

    pub fn find_subgraph_isomorphisms(&self, other: &Graph<V, E>) -> Vec<HashMap<usize, usize>> {
        let mut mappings = Vec::new();
        if self.vertices.len() < other.vertices.len() {
            return mappings;
        }
        let mut mapping = HashMap::new();
        let mut reverse_mapping = HashMap::new();
        other.vf2_all_matches(self, &mut mapping, &mut reverse_mapping, 0, true, &mut mappings);
        mappings
    }

    fn vf2_match(
        &self,
        other: &Graph<V, E>,
        mapping: &mut HashMap<usize, usize>,
        reverse_mapping: &mut HashMap<usize, usize>,
        depth: usize,
        subgraph: bool,
    ) -> bool {
        if depth == self.vertices.len() {
            return true;
        }

        let v1 = depth;
        for v2 in 0..other.vertices.len() {
            if !reverse_mapping.contains_key(&v2) && self.is_feasible(v1, v2, other, mapping, subgraph) {
                mapping.insert(v1, v2);
                reverse_mapping.insert(v2, v1);

                if self.vf2_match(other, mapping, reverse_mapping, depth + 1, subgraph) {
                    return true;
                }

                mapping.remove(&v1);
                reverse_mapping.remove(&v2);
            }
        }
        false
    }

    fn vf2_all_matches(
        &self,
        other: &Graph<V, E>,
        mapping: &mut HashMap<usize, usize>,
        reverse_mapping: &mut HashMap<usize, usize>,
        depth: usize,
        subgraph: bool,
        mappings: &mut Vec<HashMap<usize, usize>>,
    ) {
        if depth == self.vertices.len() {
            mappings.push(mapping.clone());
            return;
        }

        let v1 = depth;
        for v2 in 0..other.vertices.len() {
            if !reverse_mapping.contains_key(&v2) && self.is_feasible(v1, v2, other, mapping, subgraph) {
                mapping.insert(v1, v2);
                reverse_mapping.insert(v2, v1);

                self.vf2_all_matches(other, mapping, reverse_mapping, depth + 1, subgraph, mappings);

                mapping.remove(&v1);
                reverse_mapping.remove(&v2);
            }
        }
    }

    fn is_feasible(
        &self,
        v1: usize,
        v2: usize,
        other: &Graph<V, E>,
        mapping: &HashMap<usize, usize>,
        subgraph: bool,
    ) -> bool {
        // Semantic check
        if !self.vertices[v1].equivalent(&other.vertices[v2]) {
            return false;
        }

        // Structural check
        for (&neighbor1, edge1) in &self.edges[v1] {
            if let Some(&neighbor2_mapped) = mapping.get(&neighbor1) {
                if let Some(edge2) = other.get_edge(v2, neighbor2_mapped) {
                    if !edge1.equivalent(edge2) {
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
}

impl<V: Vertex, E: Edge> Default for Graph<V, E> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_basic() {
        let mut g = Graph::<BaseVertex, BaseEdge>::new();
        let v1 = g.add_vertex(BaseVertex::default());
        let v2 = g.add_vertex(BaseVertex::default());
        g.add_edge(v1, v2, BaseEdge::default());

        assert_eq!(g.vertices.len(), 2);
        assert!(g.has_edge(v1, v2));
        assert!(g.has_edge(v2, v1));
        assert!(g.get_edge(v1, v2).is_some());
    }

    #[test]
    fn test_remove_vertex() {
        let mut g = Graph::<BaseVertex, BaseEdge>::new();
        let v1 = g.add_vertex(BaseVertex::default());
        let v2 = g.add_vertex(BaseVertex::default());
        let v3 = g.add_vertex(BaseVertex::default());
        g.add_edge(v1, v2, BaseEdge::default());
        g.add_edge(v2, v3, BaseEdge::default());

        g.remove_vertex(v1); // v1 is gone, v2 becomes 0, v3 becomes 1
        assert_eq!(g.vertices.len(), 2);
        assert!(g.has_edge(0, 1));
    }

    #[test]
    fn test_isomorphism() {
        let mut g1 = Graph::<BaseVertex, BaseEdge>::new();
        let v1 = g1.add_vertex(BaseVertex::default());
        let v2 = g1.add_vertex(BaseVertex::default());
        g1.add_edge(v1, v2, BaseEdge::default());

        let mut g2 = Graph::<BaseVertex, BaseEdge>::new();
        let v3 = g2.add_vertex(BaseVertex::default());
        let v4 = g2.add_vertex(BaseVertex::default());
        g2.add_edge(v3, v4, BaseEdge::default());

        assert!(g1.is_isomorphic(&g2));

        let mut g3 = Graph::<BaseVertex, BaseEdge>::new();
        g3.add_vertex(BaseVertex::default());
        g3.add_vertex(BaseVertex::default());
        // No edge
        assert!(!g1.is_isomorphic(&g3));
    }

    #[test]
    fn test_connectivity_values() {
        // 0-1-2-3-4
        //   |
        //   5
        let mut g = Graph::<BaseVertex, BaseEdge>::new();
        let vertices: Vec<usize> = (0..6).map(|_| g.add_vertex(BaseVertex::default())).collect();
        g.add_edge(vertices[0], vertices[1], BaseEdge::default());
        g.add_edge(vertices[1], vertices[2], BaseEdge::default());
        g.add_edge(vertices[2], vertices[3], BaseEdge::default());
        g.add_edge(vertices[3], vertices[4], BaseEdge::default());
        g.add_edge(vertices[1], vertices[5], BaseEdge::default());

        g.update_connectivity_values();

        let expected_cv1 = [1, 3, 2, 2, 1, 1];
        let expected_cv2 = [3, 4, 5, 3, 2, 3];
        let expected_cv3 = [4, 11, 7, 7, 3, 4];

        for i in 0..6 {
            assert_eq!(g.vertices[i].connectivity1, expected_cv1[i]);
            assert_eq!(g.vertices[i].connectivity2, expected_cv2[i]);
            assert_eq!(g.vertices[i].connectivity3, expected_cv3[i]);
        }
    }

    #[test]
    fn test_split() {
        let mut g = Graph::<BaseVertex, BaseEdge>::new();
        let v: Vec<usize> = (0..6).map(|_| g.add_vertex(BaseVertex::default())).collect();
        g.add_edge(v[0], v[1], BaseEdge::default());
        g.add_edge(v[1], v[2], BaseEdge::default());
        g.add_edge(v[2], v[3], BaseEdge::default());
        g.add_edge(v[4], v[5], BaseEdge::default());

        let components = g.split();
        assert_eq!(components.len(), 2);
        let lens: Vec<usize> = components.iter().map(|c| c.vertices.len()).collect();
        assert!(lens.contains(&4));
        assert!(lens.contains(&2));
    }

    #[test]
    fn test_merge() {
        let mut g1 = Graph::<BaseVertex, BaseEdge>::new();
        let v1: Vec<usize> = (0..4).map(|_| g1.add_vertex(BaseVertex::default())).collect();
        g1.add_edge(v1[0], v1[1], BaseEdge::default());

        let mut g2 = Graph::<BaseVertex, BaseEdge>::new();
        let v2: Vec<usize> = (0..3).map(|_| g2.add_vertex(BaseVertex::default())).collect();
        g2.add_edge(v2[0], v2[1], BaseEdge::default());

        let g = g1.merge(&g2);
        assert_eq!(g.vertices.len(), 7);
    }

    #[test]
    fn test_subgraph_isomorphism() {
        let mut g1 = Graph::<BaseVertex, BaseEdge>::new();
        let v1: Vec<usize> = (0..6).map(|_| g1.add_vertex(BaseVertex::default())).collect();
        // Path graph 0-1-2-3-4-5
        for i in 0..5 {
            g1.add_edge(v1[i], v1[i + 1], BaseEdge::default());
        }

        let mut g2 = Graph::<BaseVertex, BaseEdge>::new();
        let v2: Vec<usize> = (0..2).map(|_| g2.add_vertex(BaseVertex::default())).collect();
        g2.add_edge(v2[0], v2[1], BaseEdge::default());

        assert!(g1.is_subgraph_isomorphic(&g2));
        let mappings = g1.find_subgraph_isomorphisms(&g2);
        // A single edge (g2) can be mapped to any of the 5 edges in the path (g1)
        // Each edge can be mapped in 2 directions.
        // 5 edges * 2 directions = 10 mappings.
        assert_eq!(mappings.len(), 10);
    }
}
