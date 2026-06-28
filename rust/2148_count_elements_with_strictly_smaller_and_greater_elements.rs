/// LeetCode #2148 - Count Elements With Strictly Smaller and Greater Elements
fn count_elements(nums: Vec<i32>) -> i32 {
    if nums.len() <= 2 {
        return 0;
    }

    let min = *nums.iter().min().unwrap();
    let max = *nums.iter().max().unwrap();
    if min == max {
        return 0;
    }

    nums.iter().filter(|&&value| value != min && value != max).count() as i32
}

fn main() {
    println!("{}", count_elements(vec![11, 7, 2, 15]));
}

#[cfg(test)]
mod tests {
    use super::count_elements;

    #[test]
    fn example_one() {
        assert_eq!(count_elements(vec![11, 7, 2, 15]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_elements(vec![-3, 3, 3, 90]), 2);
    }
}
