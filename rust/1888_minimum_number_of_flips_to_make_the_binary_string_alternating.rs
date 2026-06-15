/// LeetCode #1888 - Minimum Number of Flips to Make the Binary String Alternating
fn min_flips(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let target = b"01";
    let mut cnt = s
        .iter()
        .enumerate()
        .filter(|&(i, &c)| c != target[i & 1])
        .count();
    let mut ans = cnt.min(n - cnt);
    for i in 0..n {
        if s[i] != target[i & 1] {
            cnt -= 1;
        }
        if s[i] != target[(i + n) & 1] {
            cnt += 1;
        }
        ans = ans.min(cnt.min(n - cnt));
    }
    ans as i32
}

fn main() {
    println!("{}", min_flips("111000".into()));
}

#[cfg(test)]
mod tests {
    use super::min_flips;

    #[test]
    fn example_one() {
        assert_eq!(min_flips("111000".into()), 2);
    }
}
