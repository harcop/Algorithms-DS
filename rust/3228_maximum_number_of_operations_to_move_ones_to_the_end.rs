/// LeetCode #3228 - Maximum Number of Operations to Move Ones to the End
fn max_operations(s: String) -> i32 {
    let mut ans = 0;
    let mut cnt = 0;
    let bytes = s.as_bytes();
    for i in 0..bytes.len() {
        if bytes[i] == b'1' {
            cnt += 1;
        } else if i > 0 && bytes[i - 1] == b'1' {
            ans += cnt;
        }
    }
    ans
}

fn main() {
    println!("{}", max_operations("1001101".into()));
}

#[cfg(test)]
mod tests {
    use super::max_operations;

    #[test]
    fn example1() {
        assert_eq!(max_operations("1001101".into()), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(max_operations("00111".into()), 0);
    }
}
