/// LeetCode #2593 - Find Score of an Array After Marking All Elements
fn find_score(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_unstable_by(|&i, &j| nums[i].cmp(&nums[j]).then(i.cmp(&j)));
    let mut vis = vec![false; n + 2];
    let mut ans = 0i64;
    for i in idx {
        if !vis[i + 1] {
            ans += nums[i] as i64;
            vis[i] = true;
            vis[i + 2] = true;
        }
    }
    ans
}

fn main() {
    println!("{}", find_score(vec![2, 1, 3, 4, 5, 2]));
}

#[cfg(test)]
mod tests {
    use super::find_score;

    #[test]
    fn example_one() {
        assert_eq!(find_score(vec![2, 1, 3, 4, 5, 2]), 7);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_score(vec![2, 3, 5, 1, 3, 2]), 5);
    }
}
