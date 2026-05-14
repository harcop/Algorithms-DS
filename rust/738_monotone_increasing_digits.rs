/// LeetCode #738 - Monotone Increasing Digits
fn monotone_increasing_digits(n: i32) -> i32 {
    let mut s: Vec<u8> = n.to_string().into_bytes();
    let mut mark = s.len();
    for i in (0..s.len() - 1).rev() {
        if s[i] > s[i + 1] {
            mark = i;
            s[i] -= 1;
        }
    }
    for i in mark + 1..s.len() {
        s[i] = b'9';
    }
    String::from_utf8(s).unwrap().parse().unwrap()
}

fn main() {
    println!("{}", monotone_increasing_digits(10));
}

#[cfg(test)]
mod tests {
    use super::monotone_increasing_digits;

    #[test]
    fn example_one() {
        assert_eq!(monotone_increasing_digits(10), 9);
    }

    #[test]
    fn example_two() {
        assert_eq!(monotone_increasing_digits(1234), 1234);
    }
}
