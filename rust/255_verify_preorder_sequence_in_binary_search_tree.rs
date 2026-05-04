/// LeetCode #255 - Verify Preorder Sequence in Binary Search Tree
fn verify_preorder(preorder: Vec<i32>) -> bool {
    let mut stack = vec![];
    let mut lo = i32::MIN;
    for x in preorder {
        if x < lo {
            return false;
        }
        while let Some(&t) = stack.last() {
            if x > t {
                lo = stack.pop().unwrap();
            } else {
                break;
            }
        }
        stack.push(x);
    }
    true
}

fn main() {
    println!("{}", verify_preorder(vec![5, 2, 1, 3, 6]));
}

#[cfg(test)]
mod tests {
    use super::verify_preorder;

    #[test]
    fn example_one() {
        assert!(verify_preorder(vec![5, 2, 1, 3, 6]));
    }

    #[test]
    fn example_two() {
        assert!(!verify_preorder(vec![5, 2, 6, 3, 1]));
    }
}
