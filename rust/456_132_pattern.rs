/// LeetCode #456 - 132 Pattern
fn find132pattern(nums: Vec<i32>) -> bool {
    let mut stack = vec![];
    let mut third = i32::MIN;
    for x in nums.into_iter().rev() {
        if x < third {
            return true;
        }
        while let Some(&last) = stack.last() {
            if x > last {
                third = stack.pop().unwrap();
            } else {
                break;
            }
        }
        stack.push(x);
    }
    false
}

fn main() {
    println!("{}", find132pattern(vec![3, 1, 4, 2]));
}

#[cfg(test)]
mod tests {
    use super::find132pattern;

    #[test]
    fn example_one() {
        assert!(find132pattern(vec![3, 1, 4, 2]));
    }

    #[test]
    fn example_two() {
        assert!(!find132pattern(vec![1, 2, 3, 4]));
    }
}
