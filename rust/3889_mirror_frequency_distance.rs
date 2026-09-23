/// LeetCode #3889 - Mirror Frequency Distance
fn mirror_frequency_distance(s: String) -> i64 {
    let mut freq = [0i64; 256];
    for b in s.bytes() {
        freq[b as usize] += 1;
    }
    let mut ans = 0i64;
    for c in b'a'..=b'm' {
        let m = b'a' + b'z' - c;
        ans += (freq[c as usize] - freq[m as usize]).abs();
    }
    for c in b'0'..=b'4' {
        let m = b'0' + b'9' - c;
        ans += (freq[c as usize] - freq[m as usize]).abs();
    }
    ans
}

fn main() {
    println!("{}", mirror_frequency_distance("ab1z9".into()));
}

#[cfg(test)]
mod tests {
    use super::mirror_frequency_distance;

    #[test]
    fn example1() {
        assert_eq!(mirror_frequency_distance("ab1z9".into()), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(mirror_frequency_distance("4m7n".into()), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(mirror_frequency_distance("byby".into()), 0);
    }
}
