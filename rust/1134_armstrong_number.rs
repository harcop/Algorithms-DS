/// LeetCode #1134 - Armstrong Number
fn is_armstrong(n: i32) -> bool {
    let s = n.to_string();
    let k = s.len() as u32;
    let sum: i32 = s
        .bytes()
        .map(|c| (c - b'0') as i32)
        .map(|d| d.pow(k))
        .sum();
    sum == n
}

fn main() {
    println!("{}", is_armstrong(153));
}

#[cfg(test)]
mod tests {
    use super::is_armstrong;

    #[test]
    fn example_one() {
        assert!(is_armstrong(153));
    }

    #[test]
    fn example_two() {
        assert!(!is_armstrong(123));
    }
}
