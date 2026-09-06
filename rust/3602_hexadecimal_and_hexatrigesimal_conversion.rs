/// LeetCode #3602 - Hexadecimal and Hexatrigesimal Conversion
fn f(mut x: i64, k: i64) -> String {
    if x == 0 {
        return "0".into();
    }
    let digits: Vec<u8> = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_vec();
    let mut res = Vec::new();
    while x > 0 {
        res.push(digits[(x % k) as usize]);
        x /= k;
    }
    res.reverse();
    String::from_utf8(res).unwrap()
}

fn concat_hex36(n: i32) -> String {
    let n = n as i64;
    format!("{}{}", f(n * n, 16), f(n * n * n, 36))
}

fn main() {
    println!("{}", concat_hex36(13));
}

#[cfg(test)]
mod tests {
    use super::concat_hex36;

    #[test]
    fn example1() {
        assert_eq!(concat_hex36(13), "A91P1");
    }

    #[test]
    fn example2() {
        assert_eq!(concat_hex36(36), "5101000");
    }
}
