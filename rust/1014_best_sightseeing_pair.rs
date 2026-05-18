/// LeetCode #1014 - Best Sightseeing Pair
fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
    let mut best = 0i32;
    let mut peak = values[0];
    for j in 1..values.len() {
        best = best.max(peak + values[j] - j as i32);
        peak = peak.max(values[j] + j as i32);
    }
    best
}

fn main() {
    println!("{}", max_score_sightseeing_pair(vec![8, 1, 5, 2, 6]));
}

#[cfg(test)]
mod tests {
    use super::max_score_sightseeing_pair;

    #[test]
    fn example_one() {
        assert_eq!(max_score_sightseeing_pair(vec![8, 1, 5, 2, 6]), 11);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_score_sightseeing_pair(vec![1, 2]), 2);
    }
}
