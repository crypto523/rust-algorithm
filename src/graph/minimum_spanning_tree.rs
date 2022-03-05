use std::vec::Vec;

#[derive(Debug)]
pub struct Edge {
    source: i64,
    destination: i64,
    cost: i64,
}

#[derive(Debug)]
struct DSUNode {
    parent: i64,
    subtree_size: i64,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
            && self.destination == other.destination
            && self.cost == other.cost
    }
}

impl Eq for Edge {}

impl Edge {
    fn new(source: i64, destination: i64, cost: i64) -> Self {
        Self {
            source,
            destination,
            cost,
        }
    }
}

fn make_sets(number_of_vertices: i64) -> Vec<DSUNode> {
    let mut dsu_nodes: Vec<DSUNode> = Vec::with_capacity(number_of_vertices as usize);
    for i in 0..number_of_vertices {
        dsu_nodes.push(DSUNode {
            parent: i,
            subtree_size: 1,
        });
    }
    dsu_nodes
}

fn find(dsu_nodes: &mut Vec<DSUNode>, x: i64) -> i64 {
    let idx: usize = x as usize;
    if dsu_nodes[idx].parent != x {
        dsu_nodes[idx].parent = find(dsu_nodes, dsu_nodes[idx].parent);
        // subtree_size of this vertex might become invalid, but only size of
        // roots are important and used, so it doesn't matter
    }
    dsu_nodes[idx].parent
}

fn merge(dsu_nodes: &mut Vec<DSUNode>, x: i64, y: i64) {
    let mut idx_x: usize = find(dsu_nodes, x) as usize;
    let mut idx_y: usize = find(dsu_nodes, y) as usize;

    // We should make the smaller root a child of the other
    // We assume idx_x is the bigger one, and swap it if it is not
    if dsu_nodes[idx_y].subtree_size > dsu_nodes[idx_x].subtree_size {
        std::mem::swap(&mut idx_y, &mut idx_x);
    }

    dsu_nodes[idx_y].parent = idx_x as i64;
    dsu_nodes[idx_x].subtree_size += dsu_nodes[idx_y].subtree_size;
}

fn is_same_set(dsu_nodes: &mut Vec<DSUNode>, x: i64, y: i64) -> bool {
    find(dsu_nodes, x) == find(dsu_nodes, y)
}

pub fn kruskal(mut edges: Vec<Edge>, number_of_vertices: i64) -> (i64, Vec<Edge>) {
    let mut dsu_nodes: Vec<DSUNode> = make_sets(number_of_vertices);

    edges.sort_unstable_by(|a, b| a.cost.cmp(&b.cost));
    let mut total_cost: i64 = 0;
    let mut final_edges: Vec<Edge> = Vec::new();
    let mut merge_count: i64 = 0;
    for edge in edges.iter() {
        if merge_count >= number_of_vertices - 1 {
            break;
        }

        let source: i64 = edge.source;
        let destination: i64 = edge.destination;
        if !is_same_set(&mut dsu_nodes, source, destination) {
            merge(&mut dsu_nodes, source, destination);
            merge_count += 1;
            let cost: i64 = edge.cost;
            total_cost += cost;
            let final_edge: Edge = Edge::new(source, destination, cost);
            final_edges.push(final_edge);
        }
    }
    (total_cost, final_edges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seven_vertices_eleven_edges() {
        let mut edges: Vec<Edge> = Vec::new();
        edges.push(Edge::new(0, 1, 7));
        edges.push(Edge::new(0, 3, 5));
        edges.push(Edge::new(1, 2, 8));
        edges.push(Edge::new(1, 3, 9));
        edges.push(Edge::new(1, 4, 7));
        edges.push(Edge::new(2, 4, 5));
        edges.push(Edge::new(3, 4, 15));
        edges.push(Edge::new(3, 5, 6));
        edges.push(Edge::new(4, 5, 8));
        edges.push(Edge::new(4, 6, 9));
        edges.push(Edge::new(5, 6, 11));

        let number_of_vertices: i64 = 7;

        let expected_total_cost = 39;
        let mut expected_used_edges: Vec<Edge> = Vec::new();
        expected_used_edges.push(Edge::new(0, 3, 5));
        expected_used_edges.push(Edge::new(2, 4, 5));
        expected_used_edges.push(Edge::new(3, 5, 6));
        expected_used_edges.push(Edge::new(0, 1, 7));
        expected_used_edges.push(Edge::new(1, 4, 7));
        expected_used_edges.push(Edge::new(4, 6, 9));

        let (actual_total_cost, actual_final_edges) = kruskal(edges, number_of_vertices);

        assert_eq!(actual_total_cost, expected_total_cost);
        assert_eq!(actual_final_edges, expected_used_edges);
    }

    #[test]
    fn test_ten_vertices_twenty_edges() {
        let mut edges: Vec<Edge> = Vec::new();
        edges.push(Edge::new(0, 1, 3));
        edges.push(Edge::new(0, 3, 6));
        edges.push(Edge::new(0, 4, 9));
        edges.push(Edge::new(1, 2, 2));
        edges.push(Edge::new(1, 3, 4));
        edges.push(Edge::new(1, 4, 9));
        edges.push(Edge::new(2, 3, 2));
        edges.push(Edge::new(2, 5, 8));
        edges.push(Edge::new(2, 6, 9));
        edges.push(Edge::new(3, 6, 9));
        edges.push(Edge::new(4, 5, 8));
        edges.push(Edge::new(4, 9, 18));
        edges.push(Edge::new(5, 6, 7));
        edges.push(Edge::new(5, 8, 9));
        edges.push(Edge::new(5, 9, 10));
        edges.push(Edge::new(6, 7, 4));
        edges.push(Edge::new(6, 8, 5));
        edges.push(Edge::new(7, 8, 1));
        edges.push(Edge::new(7, 9, 4));
        edges.push(Edge::new(8, 9, 3));

        let number_of_vertices: i64 = 10;

        let expected_total_cost = 38;
        let mut expected_used_edges = Vec::new();
        expected_used_edges.push(Edge::new(7, 8, 1));
        expected_used_edges.push(Edge::new(1, 2, 2));
        expected_used_edges.push(Edge::new(2, 3, 2));
        expected_used_edges.push(Edge::new(0, 1, 3));
        expected_used_edges.push(Edge::new(8, 9, 3));
        expected_used_edges.push(Edge::new(6, 7, 4));
        expected_used_edges.push(Edge::new(5, 6, 7));
        expected_used_edges.push(Edge::new(2, 5, 8));
        expected_used_edges.push(Edge::new(4, 5, 8));

        let (actual_total_cost, actual_final_edges) = kruskal(edges, number_of_vertices);

        assert_eq!(actual_total_cost, expected_total_cost);
        assert_eq!(actual_final_edges, expected_used_edges);
    }
}
