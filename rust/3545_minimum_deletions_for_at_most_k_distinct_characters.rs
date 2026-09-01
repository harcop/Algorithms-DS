/// LeetCode #3545 - Minimum Deletions for At Most K Distinct Characters
fn min_deletion(s: String, k: i32) -> i32 {
    let mut cnt = [0i32; 26];
    for b in s.bytes() {
        cnt[(b - b'a') as usize] += 1;
    }
    let mut freqs: Vec<i32> = cnt.iter().copied().filter(|&x| x > 0).collect();
    freqs.sort_unstable();
    let k = k as usize;
    if freqs.len() <= k {
        0
    } else {
        freqs[..freqs.len() - k].iter().sum()
    }
}

fn main() {
    println!("{}", min_deletion("abc".into(), 2));
}

#[cfg(test)]
mod tests {
    use super::min_deletion;

    #[test]
    fn example1() {
        assert_eq!(min_deletion("abc".into(), 2), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(min_deletion("aabb".into(), 2), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(min_deletion("yyyzz".into(), 1), 2);
    }
}
