/// LeetCode #3982 - Sum of Integers with Maximum Digit Range
fn sum_max_digit_range(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut mx = -1;
    for x in nums {
        let mut y = x;
        let mut lo = 10;
        let mut hi = 0;
        while y > 0 {
            let v = y % 10;
            y /= 10;
            lo = lo.min(v);
            hi = hi.max(v);
        }
        let r = hi - lo;
        if mx < r {
            mx = r;
            ans = x;
        } else if mx == r {
            ans += x;
        }
    }
    ans
}

fn main() {
    println!("{}", sum_max_digit_range(vec![5724, 111, 350]));
}

#[cfg(test)]
mod tests {
    use super::sum_max_digit_range;

    #[test]
    fn example1() {
        assert_eq!(sum_max_digit_range(vec![5724, 111, 350]), 6074);
    }

    #[test]
    fn example2() {
        assert_eq!(sum_max_digit_range(vec![90, 900]), 990);
    }
}
