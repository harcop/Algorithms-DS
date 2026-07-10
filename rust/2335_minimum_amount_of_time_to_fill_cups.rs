/// LeetCode #2335 - Minimum Amount of Time to Fill Cups
fn fill_cups(mut amount: Vec<i32>) -> i32 {
    amount.sort_unstable();
    let dif = amount[0] + amount[1] - amount[2];
    if dif <= 0 {
        return amount[2];
    }
    (dif + 1) / 2 + amount[2]
}

fn main() {
    println!("{}", fill_cups(vec![1, 4, 2]));
}

#[cfg(test)]
mod tests {
    use super::fill_cups;

    #[test]
    fn example_one() {
        assert_eq!(fill_cups(vec![1, 4, 2]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(fill_cups(vec![5, 4, 4]), 7);
    }

    #[test]
    fn example_three() {
        assert_eq!(fill_cups(vec![5, 0, 0]), 5);
    }
}
