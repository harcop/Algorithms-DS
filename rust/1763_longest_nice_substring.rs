/// LeetCode #1763 - Longest Nice Substring
fn longest_nice_substring(s: String) -> String {
    let b = s.as_bytes();
    let n = b.len();
    let mut best_l = 0usize;
    let mut best_len = 0usize;
    for i in 0..n {
        let mut lower = [false; 26];
        let mut upper = [false; 26];
        for j in i..n {
            let c = b[j];
            if c.is_ascii_lowercase() {
                lower[(c - b'a') as usize] = true;
            } else {
                upper[(c - b'A') as usize] = true;
            }
            let mut ok = true;
            for k in 0..26 {
                if lower[k] != upper[k] {
                    ok = false;
                    break;
                }
            }
            if ok && j - i + 1 > best_len {
                best_len = j - i + 1;
                best_l = i;
            }
        }
    }
    if best_len == 0 {
        String::new()
    } else {
        String::from_utf8(b[best_l..best_l + best_len].to_vec()).unwrap()
    }
}
fn main() { println!("{}", longest_nice_substring("YazaAay".into())); }
#[cfg(test)]
mod tests {
    use super::longest_nice_substring;
    #[test]
    fn example_one() { assert_eq!(longest_nice_substring("YazaAay".into()), "aAa"); }
    #[test]
    fn example_two() { assert_eq!(longest_nice_substring("Bb".into()), "Bb"); }
}
