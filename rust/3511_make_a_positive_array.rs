/// LeetCode #3511 - Make a Positive Array
fn make_array_positive(nums: Vec<i32>) -> i32 {
    let mut l = -1i32;
    let mut ans = 0;
    let mut pre_mx = 0i64;
    let mut s = 0i64;
    for (r, &x) in nums.iter().enumerate() {
        let r = r as i32;
        s += x as i64;
        if r - l > 2 && s <= pre_mx {
            ans += 1;
            l = r;
            pre_mx = 0;
            s = 0;
        } else if r - l >= 2 {
            pre_mx = pre_mx.max(s - x as i64 - nums[r as usize - 1] as i64);
        }
    }
    ans
}

fn main() {
    println!("{}", make_array_positive(vec![-10, 15, -12]));
}

#[cfg(test)]
mod tests {
    use super::make_array_positive;

    #[test]
    fn example1() {
        assert_eq!(make_array_positive(vec![-10, 15, -12]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(make_array_positive(vec![-1, -2, 3, -1, 2, 6]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(make_array_positive(vec![1, 2, 3]), 0);
    }
}
