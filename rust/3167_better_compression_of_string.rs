/// LeetCode #3167 - Better Compression of String
use std::collections::BTreeMap;

fn better_compression(compressed: String) -> String {
    let bytes = compressed.as_bytes();
    let mut cnt: BTreeMap<u8, i64> = BTreeMap::new();
    let mut i = 0;
    let n = bytes.len();
    while i < n {
        let c = bytes[i];
        i += 1;
        let mut x = 0i64;
        while i < n && bytes[i].is_ascii_digit() {
            x = x * 10 + (bytes[i] - b'0') as i64;
            i += 1;
        }
        *cnt.entry(c).or_insert(0) += x;
    }
    let mut ans = String::new();
    for (c, v) in cnt {
        ans.push(c as char);
        ans.push_str(&v.to_string());
    }
    ans
}

fn main() {
    println!("{}", better_compression("a3c9b2c1".into()));
}

#[cfg(test)]
mod tests {
    use super::better_compression;

    #[test]
    fn example1() {
        assert_eq!(better_compression("a3c9b2c1".into()), "a3b2c10");
    }

    #[test]
    fn example2() {
        assert_eq!(better_compression("c2b3a1".into()), "a1b3c2");
    }

    #[test]
    fn example3() {
        assert_eq!(better_compression("a2b4c1".into()), "a2b4c1");
    }
}
