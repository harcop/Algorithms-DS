/// LeetCode #3563 - Lexicographically Smallest String After Adjacent Removals
fn is_consecutive(a: u8, b: u8) -> bool {
    let d = (a as i32 - b as i32).abs();
    d == 1 || d == 25
}

fn lexicographically_smallest_string(s: String) -> String {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut dp = vec![vec![String::new(); n + 1]; n + 1];
    for d in 1..=n {
        for i in 0..=n - d {
            let j = i + d;
            let mut min_s = {
                let mut t = String::new();
                t.push(bytes[i] as char);
                t.push_str(&dp[i + 1][j]);
                t
            };
            for k in i + 1..j {
                if is_consecutive(bytes[i], bytes[k]) && dp[i + 1][k].is_empty() {
                    let cand = dp[k + 1][j].clone();
                    if cand < min_s {
                        min_s = cand;
                    }
                }
            }
            dp[i][j] = min_s;
        }
    }
    dp[0][n].clone()
}

fn main() {
    println!("{}", lexicographically_smallest_string("abc".into()));
}

#[cfg(test)]
mod tests {
    use super::lexicographically_smallest_string;

    #[test]
    fn example1() {
        assert_eq!(lexicographically_smallest_string("abc".into()), "a");
    }

    #[test]
    fn example2() {
        assert_eq!(lexicographically_smallest_string("bcda".into()), "");
    }

    #[test]
    fn example3() {
        assert_eq!(lexicographically_smallest_string("zdce".into()), "zdce");
    }
}
