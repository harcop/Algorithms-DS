/// LeetCode #78 - Subsets
fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut out = vec![vec![]];
    for &num in &nums {
        let mut next = Vec::new();
        for subset in &out {
            let mut s = subset.clone();
            s.push(num);
            next.push(s);
        }
        out.extend(next);
    }
    out
}

fn main() {
    println!("{:?}", subsets(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::subsets;

    #[test]
    fn example_one() {
        let mut got = subsets(vec![1, 2, 3]);
        got.sort_unstable();
        let mut expected = vec![
            vec![],
            vec![1],
            vec![2],
            vec![3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        expected.sort_unstable();
        assert_eq!(got, expected);
    }

    #[test]
    fn example_two() {
        assert_eq!(subsets(vec![0]), vec![vec![], vec![0]]);
    }
}
