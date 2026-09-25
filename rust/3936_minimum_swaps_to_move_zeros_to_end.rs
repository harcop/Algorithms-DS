/// LeetCode #3936 - Minimum Swaps to Move Zeros to End
fn minimum_swaps(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 0 {
        return 0;
    }
    let mut ans = 0i32;
    let mut i = 0usize;
    let mut j = n - 1;
    while i < j {
        while i < n && nums[i] != 0 {
            i += 1;
        }
        while j > 0 && nums[j] == 0 {
            j -= 1;
        }
        if i >= j {
            break;
        }
        ans += 1;
        i += 1;
        j -= 1;
    }
    ans
}

fn main() {
    println!("{}", minimum_swaps(vec![0, 1, 0, 3, 12]));
}

#[cfg(test)]
mod tests {
    use super::minimum_swaps;

    #[test]
    fn example1() {
        assert_eq!(minimum_swaps(vec![0, 1, 0, 3, 12]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_swaps(vec![0, 1, 0, 2]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(minimum_swaps(vec![1, 2, 0]), 0);
    }
}
