/// LeetCode #2156 - Find Substring With Given Hash Value
fn sub_str_hash(s: String, power: i32, modulo: i32, k: i32, hash_value: i32) -> String {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let k = k as usize;
    let power = power as i64;
    let modulo = modulo as i64;
    let target = hash_value as i64;

    let mut power_k = 1i64;
    for _ in 0..k {
        power_k = power_k * power % modulo;
    }

    let mut hash = 0i64;
    let mut ans = 0usize;
    for i in (0..n).rev() {
        let value = (bytes[i] - b'a' + 1) as i64;
        hash = (hash * power + value) % modulo;
        if i + k < n {
            let removed = (bytes[i + k] - b'a' + 1) as i64 * power_k % modulo;
            hash = (hash - removed + modulo) % modulo;
        }
        if i + k <= n && hash == target {
            ans = i;
        }
    }

    s[ans..ans + k].to_string()
}

fn main() {
    println!("{}", sub_str_hash("leetcode".into(), 7, 20, 2, 0));
}

#[cfg(test)]
mod tests {
    use super::sub_str_hash;

    #[test]
    fn example_one() {
        assert_eq!(sub_str_hash("leetcode".into(), 7, 20, 2, 0), "ee");
    }

    #[test]
    fn example_two() {
        assert_eq!(sub_str_hash("fbxzaad".into(), 31, 100, 3, 32), "fbx");
    }
}
