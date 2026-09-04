/// LeetCode #1285 - Find the Start and End Number of Continuous Ranges (SQL; Rust analogue)
fn continuous_ranges(mut logs: Vec<i32>) -> Vec<(i32, i32)> {
    logs.sort();
    logs.dedup();
    if logs.is_empty() {
        return vec![];
    }
    let mut ans = Vec::new();
    let mut start = logs[0];
    let mut prev = logs[0];
    for x in logs.into_iter().skip(1) {
        if x == prev + 1 {
            prev = x;
        } else {
            ans.push((start, prev));
            start = x;
            prev = x;
        }
    }
    ans.push((start, prev));
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::continuous_ranges;

    #[test]
    fn example() {
        assert_eq!(
            continuous_ranges(vec![1, 2, 3, 7, 8, 10]),
            vec![(1, 3), (7, 8), (10, 10)]
        );
    }
}
