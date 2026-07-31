/// LeetCode #2815 - Max Pair Sum in an Array
fn max_digit(mut x: i32) -> i32 {
    let mut y = 0;
    while x > 0 {
        y = y.max(x % 10);
        x /= 10;
    }
    y
}

fn max_sum(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ans = -1;
    for i in 0..n {
        for j in i + 1..n {
            let v = nums[i] + nums[j];
            if ans < v && max_digit(nums[i]) == max_digit(nums[j]) {
                ans = v;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", max_sum(vec![2536, 1613, 3366, 162]));
}

#[cfg(test)]
mod tests {
    use super::max_sum;

    #[test]
    fn example_one() {
        assert_eq!(max_sum(vec![112, 131, 411]), -1);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_sum(vec![2536, 1613, 3366, 162]), 5902);
    }

    #[test]
    fn example_three() {
        assert_eq!(max_sum(vec![51, 71, 17, 24, 42]), 88);
    }
}
