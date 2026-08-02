/// LeetCode #2919 - Minimum Increment Operations to Make Array Beautiful
fn min_increment_operations(nums: Vec<i32>, k: i32) -> i64 {
    let mut f = 0i64;
    let mut g = 0i64;
    let mut h = 0i64;
    for x in nums {
        let next = f.min(g).min(h) + (k - x).max(0) as i64;
        f = g;
        g = h;
        h = next;
    }
    f.min(g).min(h)
}

fn main() {
    println!("{}", min_increment_operations(vec![2, 3, 0, 0, 2], 4));
}

#[cfg(test)]
mod tests {
    use super::min_increment_operations;

    #[test]
    fn example_one() {
        assert_eq!(min_increment_operations(vec![2, 3, 0, 0, 2], 4), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_increment_operations(vec![0, 1, 3, 3], 5), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(min_increment_operations(vec![1, 1, 2], 1), 0);
    }
}
