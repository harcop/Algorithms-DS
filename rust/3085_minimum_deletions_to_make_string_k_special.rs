/// LeetCode #3085 - Minimum Deletions to Make String K-Special
fn minimum_deletions(word: String, k: i32) -> i32 {
    let mut freq = [0i32; 26];
    for ch in word.chars() {
        freq[(ch as u8 - b'a') as usize] += 1;
    }
    let mut counts: Vec<i32> = freq.iter().copied().filter(|&f| f > 0).collect();
    counts.sort_unstable();
    let n = counts.len();
    let total: i32 = counts.iter().sum();
    let mut best = total;

    // Choose minimum kept frequency m among characters we keep;
    // then each kept char's freq is clamped to [m, m+k].
    for i in 0..n {
        let m = counts[i];
        let mut keep = 0;
        for &f in &counts {
            if f < m {
                continue;
            }
            keep += f.min(m + k);
        }
        best = best.min(total - keep);
    }
    best
}

fn main() {
    println!("{}", minimum_deletions("aabcaba".into(), 0));
}

#[cfg(test)]
mod tests {
    use super::minimum_deletions;

    #[test]
    fn example1() {
        assert_eq!(minimum_deletions("aabcaba".into(), 0), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_deletions("dabdcbdcdcd".into(), 2), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(minimum_deletions("aaabaaa".into(), 2), 1);
    }
}
