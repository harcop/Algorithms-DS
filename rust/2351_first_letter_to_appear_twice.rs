/// LeetCode #2351 - First Letter to Appear Twice
fn repeated_character(s: String) -> char {
    let mut vis = [false; 26];
    for &c in s.as_bytes() {
        let i = (c - b'a') as usize;
        if vis[i] {
            return c as char;
        }
        vis[i] = true;
    }
    ' '
}

fn main() {
    println!("{}", repeated_character("abccbaacz".to_string()));
}

#[cfg(test)]
mod tests {
    use super::repeated_character;

    #[test]
    fn example_one() {
        assert_eq!(repeated_character("abccbaacz".to_string()), 'c');
    }

    #[test]
    fn example_two() {
        assert_eq!(repeated_character("abcdd".to_string()), 'd');
    }
}
