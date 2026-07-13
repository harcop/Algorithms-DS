/// LeetCode #2375 - Construct Smallest Number From DI String
fn smallest_number(pattern: String) -> String {
    let n = pattern.len();
    let mut ans = String::with_capacity(n + 1);
    let mut stack = Vec::with_capacity(n + 1);

    for i in 0..=n {
        stack.push((b'1' + i as u8) as char);
        if i == n || pattern.as_bytes()[i] == b'I' {
            while let Some(c) = stack.pop() {
                ans.push(c);
            }
        }
    }

    ans
}

fn main() {
    println!("{}", smallest_number("IIIDIDDD".to_string()));
}

#[cfg(test)]
mod tests {
    use super::smallest_number;

    #[test]
    fn example_one() {
        assert_eq!(smallest_number("IIIDIDDD".to_string()), "123549876");
    }

    #[test]
    fn example_two() {
        assert_eq!(smallest_number("DDD".to_string()), "4321");
    }
}
