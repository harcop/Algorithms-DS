/// LeetCode #1764 - Form Array by Concatenating Subarrays of Another Array
fn can_choose(groups: Vec<Vec<i32>>, nums: Vec<i32>) -> bool {
    let mut i = 0usize;
    for g in groups {
        let len = g.len();
        let mut found = false;
        while i + len <= nums.len() {
            if nums[i..i + len] == g[..] {
                i += len;
                found = true;
                break;
            }
            i += 1;
        }
        if !found {
            return false;
        }
    }
    true
}
fn main() {
    println!(
        "{}",
        can_choose(vec![vec![1, 2], vec![3, 4]], vec![7, 8, 1, 2, 3, 4])
    );
}
#[cfg(test)]
mod tests {
    use super::can_choose;
    #[test]
    fn example_one() {
        assert!(can_choose(vec![vec![1, 2], vec![3, 4]], vec![7, 8, 1, 2, 3, 4]));
    }
    #[test]
    fn example_two() {
        assert!(can_choose(vec![vec![1, 2], vec![3, 4]], vec![7, 1, 2, 3, 4]));
    }
}
