/// LeetCode #90 - Subsets II
fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let mut res = vec![vec![]];
    let mut start = 0usize;

    for i in 0..nums.len() {
        let start_idx = if i > 0 && nums[i] == nums[i - 1] {
            start
        } else {
            0
        };
        start = res.len();
        let mut added = Vec::new();
        for j in start_idx..res.len() {
            let mut s = res[j].clone();
            s.push(nums[i]);
            added.push(s);
        }
        res.extend(added);
    }
    res
}

fn main() {
    println!("{:?}", subsets_with_dup(vec![1, 2, 2]));
}

#[cfg(test)]
mod tests {
    use super::subsets_with_dup;

    fn normalize(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        v.sort_unstable();
        v
    }

    #[test]
    fn example_one() {
        let got = normalize(subsets_with_dup(vec![1, 2, 2]));
        let expected = normalize(vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 2],
            vec![2],
            vec![2, 2],
        ]);
        assert_eq!(got, expected);
    }

    #[test]
    fn example_two() {
        assert_eq!(subsets_with_dup(vec![0]), vec![vec![], vec![0]]);
    }
}
