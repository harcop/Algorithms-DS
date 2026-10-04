/// LeetCode #4063 - Longest Subarray Divisible by K with At Most One Negation I
fn longest_subarray(nums: Vec<i32>, k: i32) -> i32 {
    fn scan(nums: &[i32], k: i32, skip: i32) -> i32 {
        let mut first = std::collections::HashMap::new();
        first.insert(0, -1);
        let mut sum = 0i32;
        let mut best = 0i32;
        for (i, &x) in nums.iter().enumerate() {
            let v = if i as i32 == skip { -x } else { x };
            sum = (sum + v) % k;
            if sum < 0 {
                sum += k;
            }
            if let Some(&j) = first.get(&sum) {
                best = best.max(i as i32 - j);
            } else {
                first.insert(sum, i as i32);
            }
        }
        best
    }

    let mut ans = scan(&nums, k, -1);
    for i in 0..nums.len() {
        ans = ans.max(scan(&nums, k, i as i32));
    }
    ans
}

fn main() {
    println!("{}", longest_subarray(vec![4, 1, 2], 3));
}

#[cfg(test)]
mod tests {
    use super::longest_subarray;

    #[test]
    fn example1() {
        assert_eq!(longest_subarray(vec![4, 1, 2], 3), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(longest_subarray(vec![5, 3, 4], 7), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(longest_subarray(vec![2, 2, 5], 6), 2);
    }
}
