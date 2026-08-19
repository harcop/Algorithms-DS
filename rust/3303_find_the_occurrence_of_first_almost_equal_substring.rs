/// LeetCode #3303 - Find the Occurrence of First Almost Equal Substring
fn z_function(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    let mut l = 0;
    let mut r = 0;
    for i in 1..n {
        if i < r {
            z[i] = (r - i).min(z[i - l]);
        }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }
    z
}

fn min_starting_index(s: String, pattern: String) -> i32 {
    let s = s.as_bytes();
    let p = pattern.as_bytes();
    let m = s.len();
    let n = p.len();
    let mut t1 = Vec::with_capacity(n + m);
    t1.extend_from_slice(p);
    t1.extend_from_slice(s);
    let mut t2 = Vec::with_capacity(n + m);
    t2.extend(p.iter().rev().copied());
    t2.extend(s.iter().rev().copied());
    let z1 = z_function(&t1);
    let z2 = z_function(&t2);
    for i in 0..=m - n {
        if z1[n + i] + z2[m - i] >= n - 1 {
            return i as i32;
        }
    }
    -1
}

fn main() {
    println!(
        "{}",
        min_starting_index("abcdefg".into(), "bcdffg".into())
    );
}

#[cfg(test)]
mod tests {
    use super::min_starting_index;

    #[test]
    fn example1() {
        assert_eq!(min_starting_index("abcdefg".into(), "bcdffg".into()), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(min_starting_index("ababbababa".into(), "bacaba".into()), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(min_starting_index("abcd".into(), "dba".into()), -1);
    }

    #[test]
    fn example4() {
        assert_eq!(min_starting_index("dde".into(), "d".into()), 0);
    }
}
