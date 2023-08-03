use std::cmp::max;

fn main() {
    let items: Vec<usize> = vec![1, 3];

    println!("Total weight of picked items = {}", knapsack(&items, 2));
}

fn knapsack(items: &Vec<usize>, capacity: usize) -> usize {
    let mut dp: Vec<Vec<usize>> = Default::default();

    for i in 0..items.len() {
        for w in 0..(capacity + 1) {
            if w == 0 {
                dp.push(Vec::with_capacity(capacity + 1));
                dp[i].push(0);
                continue;
            }

            let previous = if i == 0 { 0 } else { dp[i - 1][w] };
            let current_with_previous = {
                if i == 0 {
                    items[i]
                } else if items[i] > w {
                    dp[i - 1][w]
                } else {
                    items[i] + dp[i - 1][w - items[i]]
                }
            };

            dp[i].push(max(previous, current_with_previous));
        }
    }

    dp[items.len() - 1][capacity]
}
