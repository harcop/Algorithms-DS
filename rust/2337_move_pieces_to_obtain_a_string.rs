/// LeetCode #2337 - Move Pieces to Obtain a String
fn can_change(start: String, target: String) -> bool {
    let start = start.as_bytes();
    let target = target.as_bytes();
    let n = start.len();
    let mut i = 0usize;
    let mut j = 0usize;

    loop {
        while i < n && start[i] == b'_' {
            i += 1;
        }
        while j < n && target[j] == b'_' {
            j += 1;
        }
        if i == n && j == n {
            return true;
        }
        if i == n || j == n || start[i] != target[j] {
            return false;
        }
        if start[i] == b'L' && i < j {
            return false;
        }
        if start[i] == b'R' && i > j {
            return false;
        }
        i += 1;
        j += 1;
    }
}

fn main() {
    println!(
        "{}",
        can_change("_L__R__R_".to_string(), "L______RR".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::can_change;

    #[test]
    fn example_one() {
        assert!(can_change("_L__R__R_".to_string(), "L______RR".to_string()));
    }

    #[test]
    fn example_two() {
        assert!(!can_change("R_L_".to_string(), "__LR".to_string()));
    }

    #[test]
    fn example_three() {
        assert!(!can_change("_R".to_string(), "R_".to_string()));
    }
}
