/// LeetCode #3468 - Find the Number of Copy Arrays
fn count_arrays(original: Vec<i32>, bounds: Vec<Vec<i32>>) -> i32 {
    let mut mn = bounds[0][0];
    let mut mx = bounds[0][1];
    for i in 1..original.len() {
        let diff = original[i] - original[i - 1];
        mn = (mn + diff).max(bounds[i][0]);
        mx = (mx + diff).min(bounds[i][1]);
    }
    (mx - mn + 1).max(0)
}

fn main() {
    println!(
        "{}",
        count_arrays(
            vec![1, 2, 3, 4],
            vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::count_arrays;

    #[test]
    fn example1() {
        assert_eq!(
            count_arrays(
                vec![1, 2, 3, 4],
                vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]
            ),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            count_arrays(
                vec![1, 2, 3, 4],
                vec![vec![1, 10], vec![2, 9], vec![3, 8], vec![4, 7]]
            ),
            4
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            count_arrays(
                vec![1, 2, 1, 2],
                vec![vec![1, 1], vec![2, 3], vec![3, 3], vec![2, 3]]
            ),
            0
        );
    }
}
