use petgraph::unionfind::UnionFind;

#[derive(Debug)]
struct Edge {
    from: usize,
    to: usize,
    weight: usize,
}

fn main() {
    let edges = vec![
        Edge { from: 0, to: 1, weight: 15 },
        Edge { from: 0, to: 4, weight: 1 },
        Edge { from: 1, to: 2, weight: 3 },
        Edge { from: 1, to: 3, weight: 1 },
        Edge { from: 2, to: 3, weight: 2 },
        Edge { from: 2, to: 4, weight: 14 },
        Edge { from: 3, to: 4, weight: 12 },
    ];

    println!("{:?}", cluster(edges, 5, 2));
}

fn cluster(mut edges: Vec<Edge>, vertex_count: usize, desired_cluster_count: usize) -> Vec<usize> {
    let mut cluster_count = vertex_count;
    let mut uf: UnionFind<usize> = UnionFind::new(cluster_count);

    edges.sort_unstable_by(|a, b| b.weight.cmp(&a.weight));

    while let Some(e) = edges.pop() {
        if cluster_count == desired_cluster_count {
            break;
        }

        if uf.union(e.from, e.to) {
            cluster_count -= 1;
        }
    }

    return uf.into_labeling();
}
