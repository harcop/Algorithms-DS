/// LeetCode #777 - Swap Adjacent in LR String
fn can_transform(start: String, end: String) -> bool {
    if start.len() != end.len() {
        return false;
    }
    let s: Vec<char> = start.chars().collect();
    let e: Vec<char> = end.chars().collect();
    let n = s.len();
    let mut i = 0;
    let mut j = 0;
    loop {
        while i < n && s[i] == 'X' {
            i += 1;
        }
        while j < n && e[j] == 'X' {
            j += 1;
        }
        if i == n || j == n {
            return i == n && j == n;
        }
        if s[i] != e[j] {
            return false;
        }
        if s[i] == 'L' && i < j {
            return false;
        }
        if s[i] == 'R' && i > j {
            return false;
        }
        i += 1;
        j += 1;
    }
}

fn main() {
    println!(
        "{}",
        can_transform("RXXLRXRXL".into(), "XRLXXRRLX".into())
    );
}

#[cfg(test)]
mod tests {
    use super::can_transform;

    #[test]
    fn example_one() {
        assert!(can_transform("RXXLRXRXL".into(), "XRLXXRRLX".into()));
    }

    #[test]
    fn example_two() {
        assert!(!can_transform("X".into(), "L".into()));
    }
}
