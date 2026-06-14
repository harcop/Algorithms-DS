/// LeetCode #1846 - Maximum Element After Decreasing and Rearranging
fn maximum_element_after_decrementing_and_rearranging(mut arr: Vec<i32>) -> i32 {
    arr.sort_unstable();
    arr[0] = 1;
    for i in 1..arr.len() {
        let d = 0.max(arr[i] - arr[i - 1] - 1);
        arr[i] -= d;
    }
    *arr.iter().max().unwrap()
}

fn main() {
    println!(
        "{}",
        maximum_element_after_decrementing_and_rearranging(vec![2, 2, 1, 2, 1])
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_element_after_decrementing_and_rearranging;

    #[test]
    fn example_one() {
        assert_eq!(
            maximum_element_after_decrementing_and_rearranging(vec![2, 2, 1, 2, 1]),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            maximum_element_after_decrementing_and_rearranging(vec![100, 1, 1000]),
            3
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            maximum_element_after_decrementing_and_rearranging(vec![1, 2, 3, 4, 5]),
            5
        );
    }
}
