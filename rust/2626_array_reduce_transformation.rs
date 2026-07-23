/// LeetCode #2626 - Array Reduce Transformation (JS problem; Rust fold analogue)
fn reduce(nums: &[i32], fn_: fn(i32, i32) -> i32, init: i32) -> i32 {
    let mut acc = init;
    for &x in nums {
        acc = fn_(acc, x);
    }
    acc
}

fn main() {
    println!("{}", reduce(&[1, 2, 3, 4], |a, c| a + c, 0));
}

#[cfg(test)]
mod tests {
    use super::reduce;

    #[test]
    fn example_one() {
        assert_eq!(reduce(&[1, 2, 3, 4], |a, c| a + c, 0), 10);
    }

    #[test]
    fn example_two() {
        assert_eq!(reduce(&[1, 2, 3, 4], |a, c| a + c * c, 100), 130);
    }

    #[test]
    fn example_three() {
        assert_eq!(reduce(&[], |a, _| a, 25), 25);
    }
}
