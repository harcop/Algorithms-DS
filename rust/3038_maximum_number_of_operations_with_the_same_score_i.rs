/// LeetCode #3038 - Maximum Number of Operations With the Same Score I
fn max_operations(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return 0;
    }
    let score = nums[0] + nums[1];
    let mut ans = 1;
    let mut i = 2usize;
    while i + 1 < nums.len() {
        if nums[i] + nums[i + 1] == score {
            ans += 1;
            i += 2;
        } else {
            break;
        }
    }
    ans
}

fn main() {
    println!("{}", max_operations(vec![3, 2, 1, 4, 5]));
}

#[cfg(test)]
mod tests {
    use super::max_operations;

    #[test]
    fn example1() {
        assert_eq!(max_operations(vec![3, 2, 1, 4, 5]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_operations(vec![1, 5, 3, 3, 4, 1, 3, 2, 2, 3]),
            2
        );
    }

    #[test]
    fn example3() {
        assert_eq!(max_operations(vec![5, 3]), 1);
    }
}
