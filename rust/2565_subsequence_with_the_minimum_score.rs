/// LeetCode #2565 - Subsequence With the Minimum Score
fn minimum_score(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let m = s.len();
    let n = t.len();
    let mut f = vec![i32::MAX / 2; n];
    let mut g = vec![-1; n];

    let mut j = 0usize;
    for i in 0..m {
        if j < n && s[i] == t[j] {
            f[j] = i as i32;
            j += 1;
        }
    }

    j = n;
    for i in (0..m).rev() {
        if j > 0 && s[i] == t[j - 1] {
            j -= 1;
            g[j] = i as i32;
        }
    }

    let check = |len: usize| -> bool {
        for k in 0..n {
            let i = k as i32 - 1;
            let jj = k + len;
            let left = if i >= 0 { f[i as usize] } else { -1 };
            let right = if jj < n { g[jj] } else { (m + 1) as i32 };
            if left < right {
                return true;
            }
        }
        false
    };

    let mut lo = 0usize;
    let mut hi = n;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if check(mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo as i32
}

fn main() {
    println!("{}", minimum_score("abacaba".to_string(), "bzaa".to_string()));
}

#[cfg(test)]
mod tests {
    use super::minimum_score;

    #[test]
    fn example_one() {
        assert_eq!(
            minimum_score("abacaba".to_string(), "bzaa".to_string()),
            1
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_score("cde".to_string(), "xyz".to_string()), 3);
    }
}
