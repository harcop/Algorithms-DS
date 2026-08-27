/// LeetCode #3452 - Sum of Good Numbers
fn sum_of_good_numbers(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut ans = 0;
    for (i, &x) in nums.iter().enumerate() {
        if i >= k && x <= nums[i - k] {
            continue;
        }
        if i + k < nums.len() && x <= nums[i + k] {
            continue;
        }
        ans += x;
    }
    ans
}

fn main() {
    println!("{}", sum_of_good_numbers(vec![1, 3, 2, 1, 5, 4], 2));
}

#[cfg(test)]
mod tests {
    use super::sum_of_good_numbers;

    #[test]
    fn example1() {
        assert_eq!(sum_of_good_numbers(vec![1, 3, 2, 1, 5, 4], 2), 12);
    }

    #[test]
    fn example2() {
        assert_eq!(sum_of_good_numbers(vec![2, 1], 1), 2);
    }
}
