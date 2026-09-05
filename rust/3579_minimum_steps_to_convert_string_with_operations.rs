/// LeetCode #3579 - Minimum Steps to Convert String with Operations
fn calc(word1: &[u8], word2: &[u8], l: usize, r: usize, rev: bool) -> i32 {
    let mut cnt = [[0i32; 26]; 26];
    let mut res = 0;
    for i in l..=r {
        let j = if rev { r - (i - l) } else { i };
        let a = (word1[j] - b'a') as usize;
        let b = (word2[i] - b'a') as usize;
        if a != b {
            if cnt[b][a] > 0 {
                cnt[b][a] -= 1;
            } else {
                cnt[a][b] += 1;
                res += 1;
            }
        }
    }
    res
}

fn min_operations(word1: String, word2: String) -> i32 {
    let n = word1.len();
    let w1 = word1.as_bytes();
    let w2 = word2.as_bytes();
    let mut f = vec![i32::MAX / 2; n + 1];
    f[0] = 0;
    for i in 1..=n {
        for j in 0..i {
            let t = calc(w1, w2, j, i - 1, false).min(1 + calc(w1, w2, j, i - 1, true));
            f[i] = f[i].min(f[j] + t);
        }
    }
    f[n]
}

fn main() {
    println!("{}", min_operations("abcdf".into(), "dacbe".into()));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations("abcdf".into(), "dacbe".into()), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations("abceded".into(), "baecfef".into()), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(min_operations("abcdef".into(), "fedabc".into()), 2);
    }
}
