/// LeetCode #2419 - Longest Subarray With Maximum Bitwise AND
fn longest_subarray(nums: Vec<i32>) -> i32 {
    let target = *nums.iter().max().unwrap();
    let mut ans = 0;
    let mut cur = 0;

    for num in nums {
        if num == target {
            cur += 1;
            ans = ans.max(cur);
        } else {
            cur = 0;
        }
    }

    ans
}

fn main() {
    println!("{}", longest_subarray(vec![1, 2, 3, 3, 2, 2]));
}

#[cfg(test)]
mod tests {
    use super::longest_subarray;

    #[test]
    fn example_one() {
        assert_eq!(longest_subarray(vec![1, 2, 3, 3, 2, 2]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_subarray(vec![1, 2, 3, 4]), 1);
    }
}
