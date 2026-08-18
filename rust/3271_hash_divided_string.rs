/// LeetCode #3271 - Hash Divided String
fn string_hash(s: String, k: i32) -> String {
    let k = k as usize;
    let b = s.as_bytes();
    let mut ans = String::new();
    for i in (0..b.len()).step_by(k) {
        let mut t = 0;
        for j in i..i + k {
            t += (b[j] - b'a') as i32;
        }
        ans.push((b'a' + (t % 26) as u8) as char);
    }
    ans
}

fn main() {
    println!("{}", string_hash("abcd".into(), 2));
}

#[cfg(test)]
mod tests {
    use super::string_hash;

    #[test]
    fn example1() {
        assert_eq!(string_hash("abcd".into(), 2), "bf");
    }

    #[test]
    fn example2() {
        assert_eq!(string_hash("mxz".into(), 3), "i");
    }
}
