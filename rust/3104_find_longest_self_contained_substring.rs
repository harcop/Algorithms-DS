/// LeetCode #3104 - Find Longest Self-Contained Substring
fn max_substring_length(s: String) -> i32 {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut first = [-1i32; 26];
    let mut last = [0i32; 26];
    for (i, &c) in bytes.iter().enumerate() {
        let j = (c - b'a') as usize;
        if first[j] == -1 {
            first[j] = i as i32;
        }
        last[j] = i as i32;
    }
    let mut ans = -1;
    for k in 0..26 {
        let i = first[k];
        if i == -1 {
            continue;
        }
        let mut mx = last[k];
        for j in i as usize..n {
            let a = first[(bytes[j] - b'a') as usize];
            let b = last[(bytes[j] - b'a') as usize];
            if a < i {
                break;
            }
            mx = mx.max(b);
            if mx == j as i32 && j - i as usize + 1 < n {
                ans = ans.max((j - i as usize + 1) as i32);
            }
        }
    }
    ans
}

fn main() {
    println!("{}", max_substring_length("abba".into()));
}

#[cfg(test)]
mod tests {
    use super::max_substring_length;

    #[test]
    fn example1() {
        assert_eq!(max_substring_length("abba".into()), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(max_substring_length("abab".into()), -1);
    }

    #[test]
    fn example3() {
        assert_eq!(max_substring_length("abacd".into()), 4);
    }
}
