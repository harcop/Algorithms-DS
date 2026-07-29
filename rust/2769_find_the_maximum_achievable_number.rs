/// LeetCode #2769 - Find the Maximum Achievable Number
fn the_maximum_achievable_x(num: i32, t: i32) -> i32 {
    num + t * 2
}

fn main() {
    println!("{}", the_maximum_achievable_x(4, 1));
}

#[cfg(test)]
mod tests {
    use super::the_maximum_achievable_x;

    #[test]
    fn example_one() {
        assert_eq!(the_maximum_achievable_x(4, 1), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(the_maximum_achievable_x(3, 2), 7);
    }
}
