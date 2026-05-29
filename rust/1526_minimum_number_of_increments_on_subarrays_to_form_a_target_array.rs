/// LeetCode #1526 - Minimum Number Of Increments On Subarrays To Form A Target Array
fn min_number_operations(target: Vec<i32>) -> i32 {
    let mut prev = 0i32;
    let mut ans = 0i32;
    for &x in &target {
        if x > prev {
            ans += x - prev;
        }
        prev = x;
    }
    ans
}

fn main() {
    println!("{}", min_number_operations(vec![1, 2, 3, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::min_number_operations;

    #[test]
    fn example_one() {
        assert_eq!(min_number_operations(vec![1, 2, 3, 2, 1]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_number_operations(vec![3, 1, 1, 3]), 5);
    }
}
