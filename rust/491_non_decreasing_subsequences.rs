/// LeetCode #491 - Non-decreasing Subsequences
use std::collections::HashSet;

fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans = Vec::new();
    let mut path = Vec::new();
    dfs(0, &nums, &mut path, &mut ans);
    ans
}

fn dfs(start: usize, nums: &[i32], path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
    if path.len() >= 2 {
        ans.push(path.clone());
    }
    let mut used = HashSet::new();
    for i in start..nums.len() {
        if used.contains(&nums[i]) {
            continue;
        }
        if path.last().map_or(true, |&x| nums[i] >= x) {
            used.insert(nums[i]);
            path.push(nums[i]);
            dfs(i + 1, nums, path, ans);
            path.pop();
        }
    }
}

fn main() {
    println!("{:?}", find_subsequences(vec![4, 6, 7, 7]));
}

#[cfg(test)]
mod tests {
    use super::find_subsequences;
    use std::collections::HashSet;

    fn as_set(v: Vec<Vec<i32>>) -> HashSet<Vec<i32>> {
        v.into_iter().collect()
    }

    #[test]
    fn example_one() {
        let got = as_set(find_subsequences(vec![4, 6, 7, 7]));
        let want = as_set(vec![
            vec![4, 6],
            vec![4, 6, 7],
            vec![4, 6, 7, 7],
            vec![4, 7],
            vec![4, 7, 7],
            vec![6, 7],
            vec![6, 7, 7],
            vec![7, 7],
        ]);
        assert_eq!(got, want);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_subsequences(vec![4, 4, 3, 2, 1]), vec![vec![4, 4]]);
    }
}
