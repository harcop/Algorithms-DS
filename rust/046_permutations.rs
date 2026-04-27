/// LeetCode #46 - Permutations
fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    let mut out = Vec::new();
    backtrack(0, &mut nums, &mut out);
    out
}

fn backtrack(start: usize, nums: &mut Vec<i32>, out: &mut Vec<Vec<i32>>) {
    if start == nums.len() {
        out.push(nums.clone());
        return;
    }
    for i in start..nums.len() {
        nums.swap(start, i);
        backtrack(start + 1, nums, out);
        nums.swap(start, i);
    }
}

fn main() {
    println!("{:?}", permute(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::permute;

    fn normalize(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        v.sort_unstable();
        v
    }

    #[test]
    fn example_one() {
        let got = normalize(permute(vec![1, 2, 3]));
        let expected = normalize(vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ]);
        assert_eq!(got, expected);
    }

    #[test]
    fn example_two() {
        assert_eq!(permute(vec![0, 1]).len(), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(permute(vec![1]), vec![vec![1]]);
    }
}
