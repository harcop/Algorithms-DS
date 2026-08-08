/// LeetCode #3090 - Maximum Length Substring With Two Occurrences
fn maximum_length_substring(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut best = 0;
    let mut left = 0usize;
    let mut freq = [0i32; 26];

    for right in 0..bytes.len() {
        let c = (bytes[right] - b'a') as usize;
        freq[c] += 1;
        while freq[c] > 2 {
            let l = (bytes[left] - b'a') as usize;
            freq[l] -= 1;
            left += 1;
        }
        best = best.max((right - left + 1) as i32);
    }
    best
}

fn main() {
    println!("{}", maximum_length_substring("bcbbbcba".into()));
}

#[cfg(test)]
mod tests {
    use super::maximum_length_substring;

    #[test]
    fn example1() {
        assert_eq!(maximum_length_substring("bcbbbcba".into()), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_length_substring("aaaa".into()), 2);
    }
}
