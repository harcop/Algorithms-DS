/// LeetCode #2310 - Sum of Numbers With Units Digit K
fn minimum_numbers(num: i32, k: i32) -> i32 {
    if num == 0 {
        return 0;
    }
    for i in 1..=10 {
        if (k * i) % 10 == num % 10 && k * i <= num {
            return i;
        }
    }
    -1
}

fn main() {
    println!("{}", minimum_numbers(58, 9));
}

#[cfg(test)]
mod tests {
    use super::minimum_numbers;

    #[test]
    fn example_one() {
        assert_eq!(minimum_numbers(58, 9), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_numbers(37, 2), -1);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_numbers(0, 7), 0);
    }
}
