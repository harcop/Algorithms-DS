/// LeetCode #3714 - Longest Balanced Substring II
use std::collections::HashMap;

fn longest_balanced(s: String) -> i32 {
    let bytes = s.as_bytes();

    fn count1(bytes: &[u8]) -> i32 {
        let mut result = 0;
        let mut cnt = 0;
        for i in 0..bytes.len() {
            cnt += 1;
            if i + 1 == bytes.len() || bytes[i + 1] != bytes[i] {
                result = result.max(cnt);
                cnt = 0;
            }
        }
        result
    }

    fn count2(bytes: &[u8], a: u8, b: u8) -> i32 {
        let mut result = 0i32;
        let mut cnt = 0i32;
        let mut lookup: HashMap<i32, i32> = HashMap::new();
        lookup.insert(0, -1);
        for (i, &x) in bytes.iter().enumerate() {
            let i = i as i32;
            if x == a {
                cnt += 1;
            } else if x == b {
                cnt -= 1;
            } else {
                cnt = 0;
                lookup.clear();
                lookup.insert(0, i);
                continue;
            }
            if let Some(&prev) = lookup.get(&cnt) {
                result = result.max(i - prev);
            } else {
                lookup.insert(cnt, i);
            }
        }
        result
    }

    fn count3(bytes: &[u8]) -> i32 {
        let mut result = 0i32;
        let mut a = 0i32;
        let mut b = 0i32;
        let mut lookup: HashMap<(i32, i32), i32> = HashMap::new();
        lookup.insert((0, 0), -1);
        for (i, &x) in bytes.iter().enumerate() {
            let i = i as i32;
            match x {
                b'a' => a += 1,
                b'b' => b += 1,
                _ => {
                    a -= 1;
                    b -= 1;
                }
            }
            if let Some(&prev) = lookup.get(&(a, b)) {
                result = result.max(i - prev);
            } else {
                lookup.insert((a, b), i);
            }
        }
        result
    }

    count1(bytes)
        .max(count2(bytes, b'a', b'b'))
        .max(count2(bytes, b'b', b'c'))
        .max(count2(bytes, b'c', b'a'))
        .max(count3(bytes))
}

fn main() {
    println!("{}", longest_balanced("abbac".into()));
}

#[cfg(test)]
mod tests {
    use super::longest_balanced;

    #[test]
    fn example1() {
        assert_eq!(longest_balanced("abbac".into()), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(longest_balanced("aabcc".into()), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(longest_balanced("aba".into()), 2);
    }
}
