/// LeetCode #3458 - Select K Disjoint Special Substrings
fn max_substring_length(s: String, k: i32) -> bool {
    if k == 0 {
        return true;
    }
    let s = s.as_bytes();
    let n = s.len();
    let mut first = [n; 26];
    let mut last = [0usize; 26];
    for (i, &c) in s.iter().enumerate() {
        let a = (c - b'a') as usize;
        if first[a] == n {
            first[a] = i;
        }
        last[a] = i;
    }
    let mut intervals = Vec::new();
    for i in 0..n {
        let a = (s[i] - b'a') as usize;
        if i != first[a] {
            continue;
        }
        let mut r = last[a];
        let mut j = i;
        let mut valid = true;
        while j <= r {
            let b = (s[j] - b'a') as usize;
            if first[b] < i {
                valid = false;
                break;
            }
            r = r.max(last[b]);
            j += 1;
        }
        if valid && !(i == 0 && r == n - 1) {
            intervals.push((i, r));
        }
    }
    intervals.sort_unstable_by_key(|&(_, r)| r);
    let mut cnt = 0;
    let mut end = -1i32;
    for (l, r) in intervals {
        if l as i32 > end {
            cnt += 1;
            end = r as i32;
        }
    }
    cnt >= k
}

fn main() {
    println!("{}", max_substring_length("abcdbaefab".into(), 2));
}

#[cfg(test)]
mod tests {
    use super::max_substring_length;

    #[test]
    fn example1() {
        assert!(max_substring_length("abcdbaefab".into(), 2));
    }

    #[test]
    fn example2() {
        assert!(!max_substring_length("cdefdc".into(), 3));
    }

    #[test]
    fn example3() {
        assert!(max_substring_length("abeabe".into(), 0));
    }
}
