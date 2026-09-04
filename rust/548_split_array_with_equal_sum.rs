/// LeetCode #548 - Split Array with Equal Sum
use std::collections::HashSet;

fn split_array(nums: Vec<i32>) -> bool {
    let n = nums.len();
    if n < 7 {
        return false;
    }
    let mut prefix = vec![0i64; n];
    prefix[0] = nums[0] as i64;
    for i in 1..n {
        prefix[i] = prefix[i - 1] + nums[i] as i64;
    }
    for j in 3..n - 3 {
        let mut seen = HashSet::new();
        for i in 1..j - 1 {
            let s1 = prefix[i - 1];
            let s2 = prefix[j - 1] - prefix[i];
            if s1 == s2 {
                seen.insert(s1);
            }
        }
        for k in j + 2..n - 1 {
            let s3 = prefix[k - 1] - prefix[j];
            let s4 = prefix[n - 1] - prefix[k];
            if s3 == s4 && seen.contains(&s3) {
                return true;
            }
        }
    }
    false
}

fn main() {
    println!("{}", split_array(vec![1, 2, 1, 2, 1, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::split_array;

    #[test]
    fn example_one() {
        assert!(split_array(vec![1, 2, 1, 2, 1, 2, 1]));
    }

    #[test]
    fn example_two() {
        assert!(!split_array(vec![1, 2, 1, 2, 1, 2, 1, 2]));
    }
}
