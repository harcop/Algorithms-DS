/// LeetCode #567 - Permutation in String
fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }
    let mut need = [0i32; 26];
    let mut win = [0i32; 26];
    for b in s1.bytes() {
        need[(b - b'a') as usize] += 1;
    }
    let s2b = s2.as_bytes();
    for i in 0..s1.len() {
        win[(s2b[i] - b'a') as usize] += 1;
    }
    if win == need {
        return true;
    }
    for i in s1.len()..s2b.len() {
        win[(s2b[i] - b'a') as usize] += 1;
        win[(s2b[i - s1.len()] - b'a') as usize] -= 1;
        if win == need {
            return true;
        }
    }
    false
}

fn main() {
    println!("{}", check_inclusion("ab".into(), "eidbaooo".into()));
}

#[cfg(test)]
mod tests {
    use super::check_inclusion;

    #[test]
    fn example_one() {
        assert!(check_inclusion("ab".into(), "eidbaooo".into()));
    }

    #[test]
    fn example_two() {
        assert!(!check_inclusion("ab".into(), "eidboaoo".into()));
    }
}
