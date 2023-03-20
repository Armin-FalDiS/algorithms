use rand::{thread_rng, Rng};
use std::collections::HashSet;

fn main() {
    let v = HashSet::from([
        "0".to_string(),
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
        "5".to_string(),
    ]);

    let e = vec![
        ("0".to_string(), "1".to_string()),
        ("1".to_string(), "2".to_string()),
        ("1".to_string(), "3".to_string()),
        ("1".to_string(), "4".to_string()),
        ("3".to_string(), "4".to_string()),
        ("4".to_string(), "5".to_string()),
    ];

    let (minimum_cut, min_cut_size)= min_cut(v, e);

    println!("{minimum_cut:?}");
    println!("{min_cut_size:?}");
}

fn min_cut(vertices: HashSet<String>, edges: Vec<(String, String)>) -> (HashSet<String>, usize) {
    let n = vertices.len() as f32;
    let trials = (n.powf(2.0) * n.log2()) as usize;

    let mut minimum_cut: HashSet<String> = Default::default();
    let mut min_cut_degree = edges.len();

    for i in 0..trials {
        let mut v = vertices.clone();
        let mut e = edges.clone();

        random_contraction(&mut v, &mut e);

        if e.len() < min_cut_degree {
            println!("New minimum cut on try #{}!", i);
            minimum_cut = v;
            min_cut_degree = e.len();
        }
    }

    (minimum_cut, min_cut_degree)
}

fn random_contraction(vertices: &mut HashSet<String>, edges: &mut Vec<(String, String)>) {
    if vertices.len() <= 2 || edges.len() == 0 {
        return;
    }

    let random_edge = edges.swap_remove(thread_rng().gen_range(0..edges.len()));

    let new_vertex = format!("{}_{}", random_edge.0, random_edge.1);

    vertices.remove(&random_edge.0);
    vertices.remove(&random_edge.1);
    vertices.insert(new_vertex.clone());

    *edges = edges
        .iter()
        .filter_map(|e| {
            if e.0 == random_edge.0 && e.1 == random_edge.1 {
                None
            } else if e.0 == random_edge.0 || e.0 == random_edge.1 {
                Some((new_vertex.clone(), e.1.clone()))
            } else if e.1 == random_edge.0 || e.1 == random_edge.1 {
                Some((e.0.clone(), new_vertex.clone()))
            } else {
                Some(e.clone())
            }
        })
        .collect();

    random_contraction(vertices, edges);
}
