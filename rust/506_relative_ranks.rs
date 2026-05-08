/// LeetCode #506 - Relative Ranks
fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let mut idx: Vec<usize> = (0..score.len()).collect();
    idx.sort_unstable_by_key(|&i| -score[i]);
    let mut out = vec![String::new(); score.len()];
    for (r, i) in idx.into_iter().enumerate() {
        out[i] = match r {
            0 => "Gold Medal".into(),
            1 => "Silver Medal".into(),
            2 => "Bronze Medal".into(),
            _ => (r + 1).to_string(),
        };
    }
    out
}

fn main() {
    println!("{:?}", find_relative_ranks(vec![5, 4, 3, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::find_relative_ranks;

    #[test]
    fn example_one() {
        assert_eq!(
            find_relative_ranks(vec![5, 4, 3, 2, 1]),
            vec![
                "Gold Medal".to_string(),
                "Silver Medal".to_string(),
                "Bronze Medal".to_string(),
                "4".to_string(),
                "5".to_string()
            ]
        );
    }
}
