/// LeetCode #537 - Complex Number Multiplication
fn complex_number_multiply(num1: String, num2: String) -> String {
    fn parse(s: &str) -> (i32, i32) {
        let s = s.trim_end_matches('i');
        let (a, b) = s.split_once('+').unwrap();
        (a.parse().unwrap(), b.parse().unwrap())
    }
    let (r1, i1) = parse(num1.trim());
    let (r2, i2) = parse(num2.trim());
    let r = r1 * r2 - i1 * i2;
    let im = r1 * i2 + i1 * r2;
    format!("{}+{}i", r, im)
}

fn main() {
    println!("{}", complex_number_multiply("1+1i".into(), "1+1i".into()));
}

#[cfg(test)]
mod tests {
    use super::complex_number_multiply;

    #[test]
    fn example_one() {
        assert_eq!(complex_number_multiply("1+1i".into(), "1+1i".into()), "0+2i");
    }
}
