/// LeetCode #161 - One Edit Distance
fn is_one_edit_distance(s: String, t: String) -> bool {
    let a = s.as_bytes();
    let b = t.as_bytes();
    if (a.len() as i32 - b.len() as i32).abs() > 1 {
        return false;
    }
    let mut i = 0usize;
    let mut j = 0usize;
    let mut edits = 0;
    while i < a.len() && j < b.len() {
        if a[i] == b[j] {
            i += 1;
            j += 1;
        } else {
            edits += 1;
            if edits > 1 {
                return false;
            }
            match a.len().cmp(&b.len()) {
                std::cmp::Ordering::Equal => {
                    i += 1;
                    j += 1;
                }
                std::cmp::Ordering::Greater => i += 1,
                std::cmp::Ordering::Less => j += 1,
            }
        }
    }
    edits + (a.len() - i).max(b.len() - j) == 1
}

fn main() {
    println!("{}", is_one_edit_distance("ab".into(), "acb".into()));
}

#[cfg(test)]
mod tests {
    use super::is_one_edit_distance;

    #[test]
    fn example_one() {
        assert!(is_one_edit_distance("ab".into(), "acb".into()));
    }

    #[test]
    fn example_two() {
        assert!(!is_one_edit_distance("cab".into(), "ad".into()));
    }
}
