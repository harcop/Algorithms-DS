/// LeetCode #507 - Perfect Number
fn check_perfect_number(num: i32) -> bool {
    if num <= 1 {
        return false;
    }
    let mut sum = 1i32;
    let mut d = 2i32;
    while d * d <= num {
        if num % d == 0 {
            sum += d;
            let other = num / d;
            if other != d {
                sum += other;
            }
        }
        d += 1;
    }
    sum == num
}

fn main() {
    println!("{}", check_perfect_number(28));
}

#[cfg(test)]
mod tests {
    use super::check_perfect_number;

    #[test]
    fn example_one() {
        assert!(check_perfect_number(28));
    }

    #[test]
    fn example_two() {
        assert!(!check_perfect_number(7));
    }
}
