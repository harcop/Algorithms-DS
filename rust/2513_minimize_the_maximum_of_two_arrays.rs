/// LeetCode #2513 - Minimize the Maximum of Two Arrays
fn minimize_set(divisor1: i32, divisor2: i32, unique_cnt1: i32, unique_cnt2: i32) -> i32 {
    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }
    let divisor1 = divisor1 as i64;
    let divisor2 = divisor2 as i64;
    let unique_cnt1 = unique_cnt1 as i64;
    let unique_cnt2 = unique_cnt2 as i64;
    let divisor = divisor1 / gcd(divisor1, divisor2) * divisor2;
    let mut left = 1i64;
    let mut right = 10_000_000_000i64;
    while left < right {
        let mid = (left + right) >> 1;
        let cnt1 = mid / divisor1 * (divisor1 - 1) + mid % divisor1;
        let cnt2 = mid / divisor2 * (divisor2 - 1) + mid % divisor2;
        let cnt = mid / divisor * (divisor - 1) + mid % divisor;
        if cnt1 >= unique_cnt1 && cnt2 >= unique_cnt2 && cnt >= unique_cnt1 + unique_cnt2 {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left as i32
}

fn main() {
    println!("{}", minimize_set(2, 7, 1, 3));
}

#[cfg(test)]
mod tests {
    use super::minimize_set;

    #[test]
    fn example_one() {
        assert_eq!(minimize_set(2, 7, 1, 3), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimize_set(3, 5, 2, 1), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimize_set(2, 4, 8, 2), 15);
    }
}
