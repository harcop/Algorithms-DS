/// LeetCode #2936 - Number of Equal Numbers Blocks (BigArray; Vec analogue)
fn count_blocks(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut i = 0usize;
    let mut ans = 0;
    while i < n {
        ans += 1;
        let x = nums[i];
        let mut lo = i + 1;
        let mut hi = n;
        while lo < hi {
            let mid = (lo + hi) / 2;
            if nums[mid] != x {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        i = lo;
    }
    ans
}

fn main() {
    println!("{}", count_blocks(&[1, 1, 1, 3, 9, 9, 9, 2, 10, 10]));
}

#[cfg(test)]
mod tests {
    use super::count_blocks;

    #[test]
    fn example_one() {
        assert_eq!(count_blocks(&[3, 3, 3, 3, 3]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_blocks(&[1, 1, 1, 3, 9, 9, 9, 2, 10, 10]), 5);
    }

    #[test]
    fn example_three() {
        assert_eq!(count_blocks(&[1, 2, 3, 4, 5, 6, 7]), 7);
    }
}
