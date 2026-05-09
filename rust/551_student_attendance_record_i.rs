/// LeetCode #551 - Student Attendance Record I
fn check_record(s: String) -> bool {
    let mut a = 0i32;
    let mut late = 0i32;
    for c in s.chars() {
        match c {
            'A' => {
                a += 1;
                late = 0;
                if a > 1 {
                    return false;
                }
            }
            'L' => {
                late += 1;
                if late > 2 {
                    return false;
                }
            }
            _ => late = 0,
        }
    }
    true
}

fn main() {
    println!("{}", check_record("PPALLP".into()));
}

#[cfg(test)]
mod tests {
    use super::check_record;

    #[test]
    fn example_one() {
        assert!(check_record("PPALLP".into()));
    }

    #[test]
    fn example_two() {
        assert!(!check_record("PPALLL".into()));
    }
}
