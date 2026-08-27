/// LeetCode #3434 - Maximum Frequency After Subarray Operation
fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
    let base = nums.iter().filter(|&&x| x == k).count() as i32;
    let mut ans = base;
    for target in 1..=50 {
        if target == k {
            continue;
        }
        let mut gain = 0;
        for &x in &nums {
            if x == target {
                gain += 1;
            } else if x == k {
                gain -= 1;
            }
            if gain < 0 {
                gain = 0;
            }
            ans = ans.max(base + gain);
        }
    }
    ans
}

fn main() {
    println!("{}", max_frequency(vec![1, 2, 3, 4, 5, 6], 1));
}

#[cfg(test)]
mod tests {
    use super::max_frequency;

    #[test]
    fn example1() {
        assert_eq!(max_frequency(vec![1, 2, 3, 4, 5, 6], 1), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_frequency(vec![10, 2, 3, 4, 5, 5, 4, 3, 2, 2], 10),
            4
        );
    }
}
