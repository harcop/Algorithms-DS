/// LeetCode #1121 - Divide Array Into Increasing Sequences
fn can_divide_into_subsequences(nums: Vec<i32>, k: i32) -> bool {
    let n = nums.len() as i32;
    let mut max_freq = 1;
    let mut cur = 1;
    for w in nums.windows(2) {
        if w[0] == w[1] {
            cur += 1;
            max_freq = max_freq.max(cur);
        } else {
            cur = 1;
        }
    }
    n >= max_freq * k
}

fn main() {
    println!("{}", can_divide_into_subsequences(vec![1, 2, 2, 3, 3, 4, 4], 3));
}

#[cfg(test)]
mod tests {
    use super::can_divide_into_subsequences;

    #[test]
    fn example_one() {
        assert!(can_divide_into_subsequences(vec![1, 2, 2, 3, 3, 4, 4], 3));
    }

    #[test]
    fn example_two() {
        assert!(!can_divide_into_subsequences(vec![5, 6, 6, 7, 8], 3));
    }
}
