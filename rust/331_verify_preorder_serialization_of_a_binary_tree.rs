/// LeetCode #331 - Verify Preorder Serialization of a Binary Tree
fn is_valid_serialization(preorder: String) -> bool {
    let mut slots = 1i32;
    for tok in preorder.split(',') {
        if slots == 0 {
            return false;
        }
        slots -= 1;
        if tok != "#" {
            slots += 2;
        }
    }
    slots == 0
}

fn main() {
    println!("{}", is_valid_serialization("9,3,4,#,#,1,#,#,2,#,6,#,#".into()));
}

#[cfg(test)]
mod tests {
    use super::is_valid_serialization;

    #[test]
    fn example_one() {
        assert!(is_valid_serialization("9,3,4,#,#,1,#,#,2,#,6,#,#".into()));
    }

    #[test]
    fn example_two() {
        assert!(!is_valid_serialization("1,#".into()));
    }
}
