/// LeetCode #3659 - Partition Array Into K-Distinct Groups
use std::collections::HashMap;

fn partition_array(nums: Vec<i32>, k: i32) -> bool {
    let n = nums.len();
    if n % k as usize != 0 {
        return false;
    }
    let m = n / k as usize;
    let mut cnt: HashMap<i32, usize> = HashMap::new();
    for x in nums {
        let c = cnt.entry(x).or_insert(0);
        *c += 1;
        if *c > m {
            return false;
        }
    }
    true
}

fn main() {
    println!("{}", partition_array(vec![1, 2, 3, 4], 2));
}

#[cfg(test)]
mod tests {
    use super::partition_array;

    #[test]
    fn example1() {
        assert!(partition_array(vec![1, 2, 3, 4], 2));
    }

    #[test]
    fn example2() {
        assert!(partition_array(vec![3, 5, 2, 2], 2));
    }

    #[test]
    fn example3() {
        assert!(!partition_array(vec![1, 5, 2, 3], 3));
    }
}
