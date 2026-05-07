/// LeetCode #316 - Remove Duplicate Letters (smallest lexicographic subsequence covering all chars)
fn remove_duplicate_letters(s: String) -> String {
    let s = s.into_bytes();
    let mut freq = vec![0i32; 26];
    for &c in &s {
        freq[(c - b'a') as usize] += 1;
    }
    let mut in_stack = [false; 26];
    let mut stack: Vec<u8> = Vec::new();
    for &c in &s {
        let i = (c - b'a') as usize;
        freq[i] -= 1;
        if in_stack[i] {
            continue;
        }
        while let Some(&last) = stack.last() {
            let li = (last - b'a') as usize;
            if c < last && freq[li] > 0 {
                stack.pop();
                in_stack[li] = false;
            } else {
                break;
            }
        }
        stack.push(c);
        in_stack[i] = true;
    }
    String::from_utf8(stack).unwrap()
}

fn main() {
    println!("{}", remove_duplicate_letters("bcabc".into()));
}

#[cfg(test)]
mod tests {
    use super::remove_duplicate_letters;

    #[test]
    fn example() {
        assert_eq!(remove_duplicate_letters("bcabc".into()), "abc");
        assert_eq!(remove_duplicate_letters("cbacdcbc".into()), "acdb");
    }
}
