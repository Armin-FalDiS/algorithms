use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::{Display, Formatter, Result},
};

#[derive(Eq, Clone)]
struct Edge {
    from: usize,
    to: usize,
    weight: usize,
}

impl Edge {
    fn new(from: usize, to: usize, weight: usize) -> Edge {
        Edge { from, to, weight }
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}--{}-->{})", self.from, self.weight, self.to)
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.weight.eq(&other.weight)
    }
}

fn main() {
    let adjacency_list = vec![
        vec![Edge::new(0, 1, 15), Edge::new(0, 4, 1)],
        vec![Edge::new(1, 0, 15), Edge::new(1, 2, 3), Edge::new(1, 3, 1)],
        vec![Edge::new(2, 1, 3), Edge::new(2, 3, 2), Edge::new(2, 4, 14)],
        vec![Edge::new(3, 1, 1), Edge::new(3, 2, 2), Edge::new(3, 4, 12)],
        vec![Edge::new(4, 0, 1), Edge::new(4, 2, 14), Edge::new(4, 3, 12)],
    ];

    let min_spanning_tree = prim(adjacency_list);

    for edge in min_spanning_tree {
        println!("{}", edge);
    }
}

fn prim(mut adjacency_list: Vec<Vec<Edge>>) -> Vec<Edge> {
    let mut spanning_tree: Vec<Edge> = Default::default();
    let mut edges: BinaryHeap<Reverse<Edge>> = BinaryHeap::new();
    let mut visited: HashSet<usize> = Default::default();

    edges.extend(get_neighbors(&mut adjacency_list, &visited, 0));
    visited.insert(0);

    while let Some(edge) = edges.pop() {
        if visited.len() == adjacency_list.len() {
            break;
        }

        let edge = edge.0;

        if !visited.insert(edge.to) {
            continue;
        }

        edges.extend(get_neighbors(&mut adjacency_list, &visited, edge.to));

        spanning_tree.push(edge);
    }

    spanning_tree
}

fn get_neighbors(
    adjacency_list: &mut Vec<Vec<Edge>>,
    visited: &HashSet<usize>,
    vertex: usize,
) -> Vec<Reverse<Edge>> {
    let mut neighbors: HashMap<usize, Edge> = Default::default();

    while let Some(e) = adjacency_list[vertex].pop() {
        if !visited.contains(&e.to) {
            if let Some(previous_edge) = neighbors.get(&e.to) {
                if &e < previous_edge {
                    neighbors.insert(e.to, e);
                }
            } else {
                neighbors.insert(e.to, e);
            }
        }
    }

    neighbors.values().map(|v| Reverse(v.clone())).collect()
}
