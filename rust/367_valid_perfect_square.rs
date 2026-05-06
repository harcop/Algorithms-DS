/// LeetCode #367 - Valid Perfect Square
fn is_perfect_square(num: i32) -> bool {
    if num < 1 {
        return false;
    }
    let mut lo: i64 = 1;
    let mut hi: i64 = num as i64;
    while lo <= hi {
        let mid = (lo + hi) / 2;
        let sq = mid * mid;
        if sq == num as i64 {
            return true;
        } else if sq < num as i64 {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    false
}

fn main() {
    println!("{}", is_perfect_square(16));
}

#[cfg(test)]
mod tests {
    use super::is_perfect_square;

    #[test]
    fn example_one() {
        assert!(is_perfect_square(16));
    }

    #[test]
    fn example_two() {
        assert!(!is_perfect_square(14));
    }
}
