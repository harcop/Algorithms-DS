/// LeetCode #395 - Longest Substring with At Least K Repeating Characters (divide & conquer)
fn longest_substring(s: String, k: i32) -> i32 {
    let k = k as usize;
    let b = s.as_bytes();

    fn dfs(slice: &[u8], k: usize) -> i32 {
        if slice.is_empty() || k == 0 {
            return 0;
        }
        let mut c = [0usize; 128];
        for &x in slice {
            c[x as usize] += 1;
        }
        for (i, &x) in slice.iter().enumerate() {
            if c[x as usize] < k {
                return dfs(&slice[..i], k).max(dfs(&slice[i + 1..], k));
            }
        }
        slice.len() as i32
    }

    dfs(b, k)
}

fn main() {
    println!("{}", longest_substring("aaabb".into(), 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lc() {
        assert_eq!(longest_substring("aaabb".into(), 3), 3);
    }
}
