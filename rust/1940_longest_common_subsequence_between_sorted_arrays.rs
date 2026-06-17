/// LeetCode #1940 - Longest Common Subsequence Between Sorted Arrays
fn longest_common_subsequence(arrays: Vec<Vec<i32>>) -> Vec<i32> {
    let mut cnt = [0i32; 101];
    for row in &arrays {
        for &x in row {
            cnt[x as usize] += 1;
        }
    }
    let m = arrays.len() as i32;
    (1..=100)
        .filter(|&x| cnt[x as usize] == m)
        .collect()
}

fn main() {
    println!(
        "{:?}",
        longest_common_subsequence(vec![vec![1, 3, 4], vec![1, 4, 7, 9]])
    );
}

#[cfg(test)]
mod tests {
    use super::longest_common_subsequence;

    #[test]
    fn example_one() {
        assert_eq!(
            longest_common_subsequence(vec![vec![1, 3, 4], vec![1, 4, 7, 9]]),
            vec![1, 4]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            longest_common_subsequence(vec![
                vec![2, 3, 6, 8],
                vec![1, 2, 3, 5, 6, 7, 10],
                vec![2, 3, 4, 6, 9]
            ]),
            vec![2, 3, 6]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            longest_common_subsequence(vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8]]),
            Vec::<i32>::new()
        );
    }
}
