/// LeetCode #2427 - Number of Common Factors
fn common_factors(a: i32, b: i32) -> i32 {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let remainder = a % b;
            a = b;
            b = remainder;
        }
        a
    }

    let common = gcd(a, b);
    let mut answer = 0;
    let mut divisor = 1;

    while divisor * divisor <= common {
        if common % divisor == 0 {
            answer += if divisor * divisor == common { 1 } else { 2 };
        }
        divisor += 1;
    }

    answer
}

fn main() {
    println!("{}", common_factors(12, 6));
}

#[cfg(test)]
mod tests {
    use super::common_factors;

    #[test]
    fn example_one() {
        assert_eq!(common_factors(12, 6), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(common_factors(25, 30), 2);
    }
}
