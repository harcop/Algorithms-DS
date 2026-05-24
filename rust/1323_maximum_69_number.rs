/// LeetCode #1323 - Maximum 69 Number
fn maximum69_number(num: i32) -> i32 {
    let mut s: Vec<char> = num.to_string().chars().collect();
    for c in s.iter_mut() {
        if *c == '6' {
            *c = '9';
            break;
        }
    }
    s.iter().collect::<String>().parse().unwrap()
}

fn main() {
    println!("{}", maximum69_number(9669));
}

#[cfg(test)]
mod tests {
    use super::maximum69_number;

    #[test]
    fn example_one() {
        assert_eq!(maximum69_number(9669), 9969);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum69_number(9996), 9999);
    }
}
