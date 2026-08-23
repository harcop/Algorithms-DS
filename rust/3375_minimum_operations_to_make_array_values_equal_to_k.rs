/// LeetCode #3375 - Minimum Operations to Make Array Values Equal to K
fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut s = std::collections::HashSet::new();
    let mut mi = i32::MAX;
    for x in nums {
        if x < k {
            return -1;
        }
        mi = mi.min(x);
        s.insert(x);
    }
    s.len() as i32 - i32::from(k == mi)
}

fn main() {
    println!("{}", min_operations(vec![5, 2, 5, 4, 5], 2));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(vec![5, 2, 5, 4, 5], 2), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![2, 1, 2], 2), -1);
    }

    #[test]
    fn example3() {
        assert_eq!(min_operations(vec![9, 7, 5, 3], 1), 4);
    }
}
