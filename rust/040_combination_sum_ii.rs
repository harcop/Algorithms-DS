/// LeetCode #40 - Combination Sum II
fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    candidates.sort_unstable();
    let mut out = Vec::new();
    let mut path = Vec::new();
    backtrack(&candidates, target, 0, &mut path, &mut out);
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
        if i > start && cands[i] == cands[i - 1] {
            continue;
        }
        if cands[i] > remain {
            break;
        }
        path.push(cands[i]);
        backtrack(cands, remain - cands[i], i + 1, path, out);
        path.pop();
    }
}

fn main() {
    println!("{:?}", combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8));
}

#[cfg(test)]
mod tests {
    use super::combination_sum2;

    fn normalize(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for inner in &mut v {
            inner.sort_unstable();
        }
        v.sort_unstable();
        v
    }

    #[test]
    fn example_one() {
        let got = normalize(combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8));
        let expected = normalize(vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]);
        assert_eq!(got, expected);
    }

    #[test]
    fn example_two() {
        let got = combination_sum2(vec![2, 5, 2, 1, 2], 5);
        let expected = vec![vec![1, 2, 2], vec![5]];
        let mut a = got;
        a.sort();
        let mut b = expected;
        b.sort();
        assert_eq!(a, b);
    }
}
