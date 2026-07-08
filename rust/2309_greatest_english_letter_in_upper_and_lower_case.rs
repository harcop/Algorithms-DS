/// LeetCode #2309 - Greatest English Letter in Upper and Lower Case
fn greatest_letter(s: String) -> String {
    let mut seen = [0u8; 26];
    for b in s.bytes() {
        if b.is_ascii_lowercase() {
            seen[(b - b'a') as usize] |= 1;
        } else {
            seen[(b - b'A') as usize] |= 2;
        }
    }
    for i in (0..26).rev() {
        if seen[i] == 3 {
            return ((b'A' + i as u8) as char).to_string();
        }
    }
    String::new()
}

fn main() {
    println!("{}", greatest_letter("lEeTcOdE".to_string()));
}

#[cfg(test)]
mod tests {
    use super::greatest_letter;

    #[test]
    fn example_one() {
        assert_eq!(greatest_letter("lEeTcOdE".to_string()), "E");
    }

    #[test]
    fn example_two() {
        assert_eq!(greatest_letter("arRAzFif".to_string()), "R");
    }

    #[test]
    fn example_three() {
        assert_eq!(greatest_letter("AbCdEfGhIjK".to_string()), "");
    }
}
