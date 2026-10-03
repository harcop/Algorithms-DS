/// LeetCode #4058 - Maximum Pulse Value After One Subarray Rotation
fn max_pulse(nums: Vec<i32>) -> i64 {
    let mut base = 0i64;
    for (i, &x) in nums.iter().enumerate() {
        if i % 2 == 0 {
            base += x as i64;
        } else {
            base -= x as i64;
        }
    }
    let mut best = 0i64;
    let mut even = 0i64;
    let mut odd = i64::MIN / 4;
    for (i, &x) in nums.iter().enumerate() {
        let sign = if i % 2 == 1 { 1 } else { -1 };
        let b = 2 * sign * x as i64;
        let next_odd = (even + b).max(b);
        let next_even = odd + b;
        odd = next_odd;
        even = next_even;
        best = best.max(even);
    }
    base + best
}

fn main() {
    println!("{}", max_pulse(vec![1, 5, 2]));
}

#[cfg(test)]
mod tests {
    use super::max_pulse;

    #[test]
    fn example1() {
        assert_eq!(max_pulse(vec![1, 5, 2]), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(max_pulse(vec![6, 4, 3]), 7);
    }

    #[test]
    fn example3() {
        assert_eq!(max_pulse(vec![9, 7]), 2);
    }
}
