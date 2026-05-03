/// LeetCode #246 - Strobogrammatic Number
fn is_strobogrammatic(num: String) -> bool {
    let map: [(u8, u8); 5] = [(b'0', b'0'), (b'1', b'1'), (b'6', b'9'), (b'8', b'8'), (b'9', b'6')];
    let b = num.as_bytes();
    let mut i = 0usize;
    let mut j = b.len().saturating_sub(1);
    while i <= j {
        let mut ok = false;
        for &(a, c) in &map {
            if b[i] == a && b[j] == c {
                ok = true;
                break;
            }
        }
        if !ok {
            return false;
        }
        i += 1;
        j = j.saturating_sub(1);
    }
    true
}

fn main() {
    println!("{}", is_strobogrammatic("69".into()));
}

#[cfg(test)]
mod tests {
    use super::is_strobogrammatic;

    #[test]
    fn example_one() {
        assert!(is_strobogrammatic("69".into()));
    }

    #[test]
    fn example_two() {
        assert!(is_strobogrammatic("88".into()));
    }

    #[test]
    fn example_three() {
        assert!(!is_strobogrammatic("962".into()));
    }
}
