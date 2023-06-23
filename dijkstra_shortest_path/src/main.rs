use std::cmp::Reverse;
use std::collections::BinaryHeap;

type WeightedAdjacencyList = Vec<Vec<(usize, usize)>>;
type DistanceVertex = Reverse<(usize, usize)>;

fn main() {
    let edges = vec![vec![(2, 1), (3, 3)], vec![(3, 2)], vec![], vec![(1, 2)]];

    println!("{:?}", dijkstra_shortest_path(edges, 0));
}

fn dijkstra_shortest_path(edges: WeightedAdjacencyList, source: usize) -> Vec<usize> {
    let mut unvisited_vertices: BinaryHeap<DistanceVertex> = BinaryHeap::new();
    let mut distances = vec![usize::MAX; edges.len()];

    let mut visited_vertices: Vec<bool> = vec![false; edges.len()];

    unvisited_vertices.push(Reverse((0, source)));

    while let Some(Reverse((distance, vertex))) = unvisited_vertices.pop() {
        visited_vertices[vertex] = true;

        if distance < distances[vertex] {
            distances[vertex] = distance;
        }

        for &(weight, destination) in edges[vertex].iter() {
            let distance_to_destination = distance + weight;

            if distance_to_destination < distances[destination] {
                distances[destination] = distance_to_destination;
            }

            if !visited_vertices[destination] {
                unvisited_vertices.push(Reverse((distance_to_destination, destination)));
            }
        }
    }

    distances
}
