/// LeetCode #2268 - Minimum Number of Keypresses
fn minimum_keypresses(s: String) -> i32 {
    let mut count = [0i32; 26];
    for c in s.bytes() {
        count[(c - b'a') as usize] += 1;
    }

    let mut freq: Vec<i32> = count.iter().copied().filter(|c| *c > 0).collect();
    freq.sort_unstable_by(|a, b| b.cmp(a));

    let mut ans = 0;
    for (i, &c) in freq.iter().enumerate() {
        ans += c * (i as i32 / 9 + 1);
    }

    ans
}

fn main() {
    println!("{}", minimum_keypresses("apple".to_string()));
}

#[cfg(test)]
mod tests {
    use super::minimum_keypresses;

    #[test]
    fn example_one() {
        assert_eq!(minimum_keypresses("apple".to_string()), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_keypresses("abcdefghijkl".to_string()), 15);
    }
}
