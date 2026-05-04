/// LeetCode #263 - Ugly Number
fn is_ugly(n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    let mut n = n;
    for p in [2, 3, 5] {
        while n % p == 0 {
            n /= p;
        }
    }
    n == 1
}

fn main() {
    println!("{}", is_ugly(6));
}

#[cfg(test)]
mod tests {
    use super::is_ugly;

    #[test]
    fn example_one() {
        assert!(is_ugly(6));
    }

    #[test]
    fn example_two() {
        assert!(is_ugly(1));
    }

    #[test]
    fn example_three() {
        assert!(!is_ugly(14));
    }
}
