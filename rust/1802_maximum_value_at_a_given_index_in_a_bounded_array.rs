/// LeetCode #1802 - Maximum Value at a Given Index in a Bounded Array
fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
    fn sum(x: i64, cnt: i64) -> i64 {
        if x >= cnt {
            (x + x - cnt + 1) * cnt / 2
        } else {
            (x + 1) * x / 2 + cnt - x
        }
    }

    let n = n as i64;
    let index = index as i64;
    let max_sum = max_sum as i64;
    let mut left = 1i64;
    let mut right = max_sum;
    while left < right {
        let mid = (left + right + 1) / 2;
        if sum(mid - 1, index) + sum(mid, n - index) <= max_sum {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    left as i32
}

fn main() {
    println!("{}", max_value(4, 2, 6));
}

#[cfg(test)]
mod tests {
    use super::max_value;

    #[test]
    fn example_one() {
        assert_eq!(max_value(4, 2, 6), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_value(6, 1, 10), 3);
    }
}
