/// LeetCode #514 - Freedom Trail
fn find_rotate_steps(ring: String, key: String) -> i32 {
    let ring = ring.into_bytes();
    let key = key.into_bytes();
    let n = ring.len();
    let mut pos = vec![Vec::new(); 26];
    for (i, &c) in ring.iter().enumerate() {
        pos[(c - b'a') as usize].push(i);
    }
    let mut dp = vec![i32::MAX; n];
    dp[0] = 0;
    for &k in &key {
        let mut ndp = vec![i32::MAX; n];
        for &j in &pos[(k - b'a') as usize] {
            for i in 0..n {
                if dp[i] == i32::MAX {
                    continue;
                }
                let diff = (i as i32 - j as i32).abs();
                let rot = diff.min(n as i32 - diff);
                ndp[j] = ndp[j].min(dp[i] + rot + 1);
            }
        }
        dp = ndp;
    }
    *dp.iter().min().unwrap()
}

fn main() {
    println!("{}", find_rotate_steps("godding".into(), "gd".into()));
}

#[cfg(test)]
mod tests {
    use super::find_rotate_steps;

    #[test]
    fn example_one() {
        assert_eq!(find_rotate_steps("godding".into(), "gd".into()), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_rotate_steps("godding".into(), "godding".into()), 13);
    }
}
