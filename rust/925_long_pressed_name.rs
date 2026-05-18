/// LeetCode #925 - Long Pressed Name
fn is_long_pressed_name(name: String, typed: String) -> bool {
    let n = name.as_bytes();
    let t = typed.as_bytes();
    let mut i = 0usize;
    let mut j = 0usize;
    while j < t.len() {
        if i < n.len() && n[i] == t[j] {
            i += 1;
            j += 1;
        } else if j > 0 && t[j] == t[j - 1] {
            j += 1;
        } else {
            return false;
        }
    }
    i == n.len()
}

fn main() {
    println!("{}", is_long_pressed_name("alex".into(), "aaleex".into()));
}

#[cfg(test)]
mod tests {
    use super::is_long_pressed_name;

    #[test]
    fn example_one() {
        assert!(is_long_pressed_name("alex".into(), "aaleex".into()));
    }

    #[test]
    fn example_two() {
        assert!(!is_long_pressed_name("saeed".into(), "ssaaedd".into()));
    }

    #[test]
    fn example_three() {
        assert!(is_long_pressed_name("laiden".into(), "laiden".into()));
    }
}
