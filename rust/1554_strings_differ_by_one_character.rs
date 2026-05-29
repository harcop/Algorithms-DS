/// LeetCode #1554 - Strings Differ By One Character
fn differ_by_one(dict: Vec<String>, s: String) -> bool {
    for t in dict {
        if t.len() != s.len() {
            continue;
        }
        let mut diff = 0;
        for (a, b) in t.bytes().zip(s.bytes()) {
            if a != b {
                diff += 1;
                if diff > 1 {
                    break;
                }
            }
        }
        if diff == 1 {
            return true;
        }
    }
    false
}

fn main() {
    println!("{}", differ_by_one(vec!["abcd".into(), "bbcd".into(), "cbcd".into()], "accd".into()));
}

#[cfg(test)]
mod tests {
    use super::differ_by_one;

    #[test]
    fn example_one() {
        assert!(differ_by_one(vec!["abcd".into(), "bbcd".into(), "cbcd".into()], "accd".into()));
    }

    #[test]
    fn example_two() {
        assert!(differ_by_one(vec!["abcd".into(), "bbcd".into(), "cbcd".into()], "abcc".into()));
    }
}
