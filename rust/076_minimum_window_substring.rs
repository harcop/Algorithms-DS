use std::collections::HashMap;

/// LeetCode #76 - Minimum Window Substring
fn min_window(s: String, t: String) -> String {
    if s.len() < t.len() || t.is_empty() {
        return String::new();
    }

    let mut need: HashMap<u8, i32> = HashMap::new();
    for &b in t.as_bytes() {
        *need.entry(b).or_insert(0) += 1;
    }
    let required = need.len();

    let bytes = s.as_bytes();
    let mut window: HashMap<u8, i32> = HashMap::new();
    let mut have = 0usize;

    let mut best_left = 0usize;
    let mut best_len = usize::MAX;

    let mut left = 0usize;
    for right in 0..bytes.len() {
        let c = bytes[right];
        *window.entry(c).or_insert(0) += 1;
        if let Some(&cnt) = need.get(&c) {
            if window[&c] == cnt {
                have += 1;
            }
        }

        while have == required && left <= right {
            if right - left + 1 < best_len {
                best_left = left;
                best_len = right - left + 1;
            }
            let cl = bytes[left];
            if let Some(&cnt) = need.get(&cl) {
                if window[&cl] == cnt {
                    have -= 1;
                }
            }
            window.entry(cl).and_modify(|x| *x -= 1);
            left += 1;
        }
    }

    if best_len == usize::MAX {
        String::new()
    } else {
        s[best_left..best_left + best_len].to_string()
    }
}

fn main() {
    println!(
        "{}",
        min_window("ADOBECODEBANC".to_string(), "ABC".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::min_window;

    #[test]
    fn example_one() {
        assert_eq!(min_window("ADOBECODEBANC".to_string(), "ABC".to_string()), "BANC");
    }

    #[test]
    fn example_two() {
        assert_eq!(min_window("a".to_string(), "a".to_string()), "a");
    }

    #[test]
    fn example_three() {
        assert_eq!(min_window("a".to_string(), "aa".to_string()), "");
    }
}
