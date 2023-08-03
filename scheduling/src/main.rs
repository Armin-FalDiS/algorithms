use std::cmp::Ordering;

#[derive(Debug)]
struct Job {
    weight: usize,
    length: usize,
}

fn main() {
    let mut jobs = vec![
        Job {
            length: 10,
            weight: 1,
        },
        Job {
            length: 1,
            weight: 10,
        },
        Job {
            length: 5,
            weight: 5,
        },
    ];

    schedule(&mut jobs);

    println!("{:?}", jobs);
}

fn schedule(jobs: &mut Vec<Job>) {
    jobs.sort_by(|a, b| {
        let a_score = a.weight as f32 / a.length as f32;
        let b_score = b.weight as f32 / b.length as f32;

        if a_score > b_score {
            Ordering::Less
        } else if a_score < b_score {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
}
