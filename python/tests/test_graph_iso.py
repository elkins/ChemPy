from chempy.graph import Edge, Graph, Vertex


def test_isomorphic_small_graph():
    g1 = Graph()
    g2 = Graph()
    a1, b1 = Vertex(), Vertex()
    e1 = Edge()
    g1.addVertex(a1)
    g1.addVertex(b1)
    g1.addEdge(a1, b1, e1)
    a2, b2 = Vertex(), Vertex()
    e2 = Edge()
    g2.addVertex(a2)
    g2.addVertex(b2)
    g2.addEdge(a2, b2, e2)
    assert g1.isIsomorphic(g2)
