/// LeetCode #1796 - Second Largest Digit in a String
fn second_highest(s: String) -> i32 {
    let mut a = -1i32;
    let mut b = -1i32;
    for ch in s.bytes() {
        if ch.is_ascii_digit() {
            let v = (ch - b'0') as i32;
            if v > a {
                b = a;
                a = v;
            } else if v > b && v < a {
                b = v;
            }
        }
    }
    b
}

fn main() {
    println!("{}", second_highest("dfa12321afd".into()));
}

#[cfg(test)]
mod tests {
    use super::second_highest;

    #[test]
    fn example_one() {
        assert_eq!(second_highest("dfa12321afd".into()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(second_highest("abc1111".into()), -1);
    }
}
