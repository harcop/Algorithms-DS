/// LeetCode #996 - Number of Squareful Arrays
fn num_squareful_perms(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let n = nums.len();
    let mut used = vec![false; n];
    let mut path = Vec::new();
    let mut count = 0i32;
    fn is_square(a: i32, b: i32) -> bool {
        let s = a + b;
        let r = (s as f64).sqrt() as i32;
        r * r == s
    }
    fn dfs(nums: &[i32], used: &mut [bool], path: &mut Vec<i32>, count: &mut i32) {
        if path.len() == nums.len() {
            *count += 1;
            return;
        }
        for i in 0..nums.len() {
            if used[i] { continue; }
            if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] { continue; }
            if !path.is_empty() && !is_square(*path.last().unwrap(), nums[i]) { continue; }
            used[i] = true;
            path.push(nums[i]);
            dfs(nums, used, path, count);
            path.pop();
            used[i] = false;
        }
    }
    dfs(&nums, &mut used, &mut path, &mut count);
    count
}

fn main() {
    println!("{}", num_squareful_perms(vec![1, 0, 0, 1, 0]));
}

#[cfg(test)]
mod tests {
    use super::num_squareful_perms;

    #[test]
    fn example_one() {
        assert_eq!(num_squareful_perms(vec![1, 0, 0, 1, 0]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_squareful_perms(vec![2, 2, 2]), 1);
    }
}
