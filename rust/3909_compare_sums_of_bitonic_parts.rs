/// LeetCode #3909 - Compare Sums of Bitonic Parts
fn compare_bitonic_sums(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut peak = 0usize;
    while peak + 1 < n && nums[peak] < nums[peak + 1] {
        peak += 1;
    }
    let asc: i64 = nums[..=peak].iter().map(|&x| x as i64).sum();
    let desc: i64 = nums[peak..].iter().map(|&x| x as i64).sum();
    if asc > desc {
        0
    } else if desc > asc {
        1
    } else {
        -1
    }
}

fn main() {
    println!("{}", compare_bitonic_sums(vec![1, 3, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::compare_bitonic_sums;

    #[test]
    fn example1() {
        assert_eq!(compare_bitonic_sums(vec![1, 3, 2, 1]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(compare_bitonic_sums(vec![2, 4, 5, 2]), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(compare_bitonic_sums(vec![1, 2, 4, 3]), -1);
    }
}
