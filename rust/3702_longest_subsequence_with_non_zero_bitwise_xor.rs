/// LeetCode #3702 - Longest Subsequence With Non-Zero Bitwise XOR
fn longest_subsequence(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i32;
    let mut xor = 0;
    let mut cnt0 = 0;
    for x in nums {
        xor ^= x;
        if x == 0 {
            cnt0 += 1;
        }
    }
    if xor != 0 {
        return n;
    }
    if cnt0 == n {
        return 0;
    }
    n - 1
}

fn main() {
    println!("{}", longest_subsequence(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::longest_subsequence;

    #[test]
    fn example1() {
        assert_eq!(longest_subsequence(vec![1, 2, 3]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(longest_subsequence(vec![2, 3, 4]), 3);
    }
}
