/// LeetCode #1400 - Construct K Palindrome Strings
fn can_construct(s: String, k: i32) -> bool {
    let k = k as usize;
    if s.len() < k {
        return false;
    }
    let mut cnt = [0i32; 26];
    for c in s.bytes() {
        cnt[(c - b'a') as usize] += 1;
    }
    let odd = cnt.iter().filter(|&&c| c % 2 == 1).count();
    odd <= k
}

fn main() {
    println!("{}", can_construct("annabelle".into(), 2));
}

#[cfg(test)]
mod tests {
    use super::can_construct;

    #[test]
    fn example_one() {
        assert!(can_construct("annabelle".into(), 2));
    }

    #[test]
    fn example_two() {
        assert!(!can_construct("leetcode".into(), 3));
    }
}

