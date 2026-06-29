/// LeetCode #2169 - Count Operations to Obtain Zero
fn count_operations(mut num1: i32, mut num2: i32) -> i32 {
    let mut ans = 0i32;
    while num1 != 0 && num2 != 0 {
        if num1 >= num2 {
            num1 -= num2;
        } else {
            num2 -= num1;
        }
        ans += 1;
    }
    ans
}

fn main() {
    println!("{}", count_operations(2, 3));
}

#[cfg(test)]
mod tests {
    use super::count_operations;

    #[test]
    fn example_one() {
        assert_eq!(count_operations(2, 3), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_operations(10, 10), 1);
    }
}
