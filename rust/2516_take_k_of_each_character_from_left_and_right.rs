/// LeetCode #2516 - Take K of Each Character From Left and Right
fn take_characters(s: String, k: i32) -> i32 {
    let bytes = s.as_bytes();
    let mut cnt = [0i32; 3];
    for &c in bytes {
        cnt[(c - b'a') as usize] += 1;
    }
    if cnt.iter().any(|&x| x < k) {
        return -1;
    }
    let n = bytes.len();
    let mut mx = 0;
    let mut j = 0;
    for i in 0..n {
        let c = (bytes[i] - b'a') as usize;
        cnt[c] -= 1;
        while cnt[c] < k {
            cnt[(bytes[j] - b'a') as usize] += 1;
            j += 1;
        }
        mx = mx.max(i + 1 - j);
    }
    (n - mx) as i32
}

fn main() {
    println!("{}", take_characters("aabaaaacaabc".to_string(), 2));
}

#[cfg(test)]
mod tests {
    use super::take_characters;

    #[test]
    fn example_one() {
        assert_eq!(take_characters("aabaaaacaabc".to_string(), 2), 8);
    }

    #[test]
    fn example_two() {
        assert_eq!(take_characters("a".to_string(), 1), -1);
    }
}
