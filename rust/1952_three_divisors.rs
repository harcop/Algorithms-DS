/// LeetCode #1952 - Three Divisors
fn is_three(n: i32) -> bool {
    if n < 4 {
        return false;
    }
    let mut cnt = 0;
    for i in 2..n {
        if n % i == 0 {
            cnt += 1;
            if cnt > 1 {
                return false;
            }
        }
    }
    cnt == 1
}

fn main() {
    println!("{}", is_three(4));
}

#[cfg(test)]
mod tests {
    use super::is_three;

    #[test]
    fn example_one() {
        assert!(!is_three(2));
    }

    #[test]
    fn example_two() {
        assert!(is_three(4));
    }
}
