/// LeetCode #674 - Longest Continuous Increasing Subsequence
fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
    if nums.is_empty() { return 0; }
    let mut best = 1i32;
    let mut cur = 1i32;
    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] { cur += 1; best = best.max(cur); } else { cur = 1; }
    }
    best
}

fn main() {
    println!("{}", find_length_of_lcis(vec![1,3,5,4,7]));
}

#[cfg(test)]
mod tests {
    use super::find_length_of_lcis;

    #[test]
    fn example_one() {
        assert_eq!(find_length_of_lcis(vec![1,3,5,4,7]), 3);
    }
}
