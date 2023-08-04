use petgraph::unionfind::UnionFind;

#[derive(Debug)]
struct Edge {
    from: usize,
    to: usize,
    weight: usize,
}

fn main() {
    let edges = vec![
        Edge { from: 0, to: 1, weight: 10 },
        Edge { from: 0, to: 3, weight: 5 },
        Edge { from: 0, to: 4, weight: 1 },
        Edge { from: 1, to: 2, weight: 4 },
        Edge { from: 2, to: 3, weight: 3 },
        Edge { from: 3, to: 4, weight: 2 },
    ];

    println!("{:?}", kruskal(edges, 5));
}

fn kruskal(mut edges: Vec<Edge>, vertex_count: usize) -> Vec<Edge> {
    let mut uf: UnionFind<usize> = UnionFind::new(vertex_count);
    let mut tree: Vec<Edge> = Default::default();

    edges.sort_unstable_by(|a, b| b.weight.cmp(&a.weight));
    
    while let Some(e) = edges.pop() {
        if !uf.equiv(e.from, e.to) {
            uf.union(e.from, e.to);
            tree.push(e);
        }
    }

    tree
}