/// LeetCode #1017 - Convert to Base -2
fn base_neg2(n: i32) -> String {
    if n == 0 {
        return "0".into();
    }
    let mut n = n as i32;
    let mut bits = Vec::new();
    while n != 0 {
        let rem = (n % 2).rem_euclid(2);
        bits.push(rem);
        n = (n - rem) / 2;
    }
    bits.iter().rev().map(|b| char::from(b'0' + *b as u8)).collect()
}

fn main() {
    println!("{}", base_neg2(2));
}

#[cfg(test)]
mod tests {
    use super::base_neg2;

    #[test]
    fn example_one() {
        assert_eq!(base_neg2(2), "110");
    }

    #[test]
    fn example_two() {
        assert_eq!(base_neg2(3), "111");
    }

    #[test]
    fn example_three() {
        assert_eq!(base_neg2(4), "100");
    }
}
