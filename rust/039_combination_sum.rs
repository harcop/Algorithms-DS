/// LeetCode #39 - Combination Sum
fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut cands: Vec<i32> = candidates;
    cands.sort_unstable();
    let mut out = Vec::new();
    let mut path = Vec::new();
    backtrack(&cands, target, 0, &mut path, &mut out);
    out
}

fn backtrack(
    cands: &[i32],
    remain: i32,
    start: usize,
    path: &mut Vec<i32>,
    out: &mut Vec<Vec<i32>>,
) {
    if remain == 0 {
        out.push(path.clone());
        return;
    }
    if remain < 0 {
        return;
    }
    for i in start..cands.len() {
        path.push(cands[i]);
        backtrack(cands, remain - cands[i], i, path, out);
        path.pop();
    }
}

fn main() {
    println!("{:?}", combination_sum(vec![2, 3, 6, 7], 7));
}

#[cfg(test)]
mod tests {
    use super::combination_sum;

    fn normalize(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for inner in &mut v {
            inner.sort_unstable();
        }
        v.sort_unstable();
        v
    }

    #[test]
    fn example_one() {
        let got = normalize(combination_sum(vec![2, 3, 6, 7], 7));
        let expected = normalize(vec![vec![2, 2, 3], vec![7]]);
        assert_eq!(got, expected);
    }

    #[test]
    fn example_two() {
        let got = normalize(combination_sum(vec![2, 3, 5], 8));
        let expected = normalize(vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]);
        assert_eq!(got, expected);
    }

    #[test]
    fn example_three() {
        let got = combination_sum(vec![2], 1);
        assert!(got.is_empty());
    }
}
