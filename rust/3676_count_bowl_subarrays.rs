/// LeetCode #3676 - Count Bowl Subarrays
fn bowl_subarrays(nums: Vec<i32>) -> i64 {
    let mut result = 0i64;
    let mut stk: Vec<usize> = Vec::new();
    for i in 0..nums.len() {
        while let Some(&top) = stk.last() {
            if nums[top] < nums[i] {
                stk.pop();
                if !stk.is_empty() {
                    result += 1;
                }
            } else {
                break;
            }
        }
        stk.push(i);
    }
    result
}

fn main() {
    println!("{}", bowl_subarrays(vec![2, 5, 3, 1, 4]));
}

#[cfg(test)]
mod tests {
    use super::bowl_subarrays;

    #[test]
    fn example1() {
        assert_eq!(bowl_subarrays(vec![2, 5, 3, 1, 4]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(bowl_subarrays(vec![5, 1, 2, 3, 4]), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(bowl_subarrays(vec![1_000_000_000, 999_999_999, 999_999_998]), 0);
    }
}
