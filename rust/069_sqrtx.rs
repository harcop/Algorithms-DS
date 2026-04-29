/// LeetCode #69 - Sqrt(x)
fn my_sqrt(x: i32) -> i32 {
    if x < 2 {
        return x;
    }
    let x = x as i64;
    let mut left = 1i64;
    let mut right = x;
    while left <= right {
        let mid = left + (right - left) / 2;
        let sq = mid * mid;
        if sq == x {
            return mid as i32;
        }
        if sq < x {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    right as i32
}

fn main() {
    println!("{}", my_sqrt(8));
}

#[cfg(test)]
mod tests {
    use super::my_sqrt;

    #[test]
    fn example_one() {
        assert_eq!(my_sqrt(4), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(my_sqrt(8), 2);
    }
}
