/// LeetCode #859 - Buddy Strings
fn buddy_strings(s: String, goal: String) -> bool {
    if s.len() != goal.len() {
        return false;
    }
    let sb = s.as_bytes();
    let gb = goal.as_bytes();
    let mut diff = Vec::new();
    for i in 0..sb.len() {
        if sb[i] != gb[i] {
            diff.push(i);
        }
    }
    if diff.is_empty() {
        let mut cnt = [0; 26];
        for &c in sb {
            cnt[(c - b'a') as usize] += 1;
            if cnt[(c - b'a') as usize] > 1 {
                return true;
            }
        }
        return false;
    }
    if diff.len() != 2 {
        return false;
    }
    sb[diff[0]] == gb[diff[1]] && sb[diff[1]] == gb[diff[0]]
}

fn main() {
    println!("{}", buddy_strings("ab".into(), "ba".into()));
}

#[cfg(test)]
mod tests {
    use super::buddy_strings;

    #[test]
    fn example_one() {
        assert!(buddy_strings("ab".into(), "ba".into()));
    }

    #[test]
    fn example_two() {
        assert!(!buddy_strings("ab".into(), "ab".into()));
    }
}
