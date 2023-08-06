fn main() {
    let x = String::from("AGGGCT");
    let y = String::from("AGGCA");
    let mismatch_penalty: usize = 2;
    let gap_penalty: usize = 3;

    println!(
        "The alignment penalty is {}",
        get_alignment_penalty(x, y, mismatch_penalty, gap_penalty)
    );
}

fn get_alignment_penalty(
    x: String,
    y: String,
    mismatch_penalty: usize,
    gap_penalty: usize,
) -> usize {
    let mut dp: Vec<Vec<usize>> = Default::default();

    let x: Vec<char> = x.chars().collect();
    let y: Vec<char> = y.chars().collect();

    for i in 0..x.len() + 1 {
        for j in 0..y.len() + 1 {
            if j == 0 {
                dp.push(Vec::with_capacity(j + 1));
                dp[i].push(i * gap_penalty);
            } else if i == 0 {
                dp[i].push(j * gap_penalty);
            } else {
                let x_gap_penalty = dp[i - 1][j] + gap_penalty;
                let y_gap_penalty = dp[i][j - 1] + gap_penalty;
                let no_gap_penalty = {
                    let matching_penalty = if x[i - 1].eq(&y[j - 1]) {
                        0
                    } else {
                        mismatch_penalty
                    };

                    dp[i - 1][j - 1] + matching_penalty
                };

                let min_penalty = x_gap_penalty.min(y_gap_penalty).min(no_gap_penalty);

                dp[i].push(min_penalty);
            }
        }
    }

    return dp[x.len()][y.len()];
}
