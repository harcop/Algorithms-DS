/// LeetCode #1447 - Simplified Fractions
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 { let t = a % b; a = b; b = t; }
    a
}
fn simplified_fractions(n: i32) -> Vec<String> {
    let mut res = Vec::new();
    for d in 2..=n {
        for num in 1..d {
            if gcd(num, d) == 1 { res.push(format!("{}/{}", num, d)); }
        }
    }
    res
}
fn main() { println!("{:?}", simplified_fractions(2)); }
#[cfg(test)]
mod tests {
    use super::simplified_fractions;
    #[test]
    fn example_one() { assert_eq!(simplified_fractions(2), vec!["1/2".to_string()]); }
    #[test]
    fn example_two() {
        let got = simplified_fractions(4);
        let exp = vec!["1/2".to_string(),"1/3".to_string(),"2/3".to_string(),"1/4".to_string(),"3/4".to_string()];
        assert_eq!(got, exp);
    }
}