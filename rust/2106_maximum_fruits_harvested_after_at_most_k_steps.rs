/// LeetCode #2106 - Maximum Fruits Harvested After at Most K Steps
fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
    let n = fruits.len();
    fn steps(left: i32, right: i32, start: i32) -> i32 {
        if right <= start {
            start - left
        } else if left >= start {
            right - start
        } else {
            (start - left).min(right - start) + right - left
        }
    }

    let mut ans = 0;
    let mut sum = 0;
    let mut left = 0usize;
    for right in 0..n {
        sum += fruits[right][1];
        while left <= right && steps(fruits[left][0], fruits[right][0], start_pos) > k {
            sum -= fruits[left][1];
            left += 1;
        }
        ans = ans.max(sum);
    }
    ans
}

fn main() {
    println!(
        "{}",
        max_total_fruits(
            vec![vec![2, 8], vec![6, 3], vec![8, 6]],
            5,
            4,
        )
    );
}

#[cfg(test)]
mod tests {
    use super::max_total_fruits;

    #[test]
    fn example_one() {
        assert_eq!(
            max_total_fruits(
                vec![vec![2, 8], vec![6, 3], vec![8, 6]],
                5,
                4,
            ),
            9
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            max_total_fruits(
                vec![vec![0, 9], vec![4, 1], vec![5, 7], vec![6, 2], vec![7, 4], vec![10, 9]],
                5,
                4,
            ),
            14
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            max_total_fruits(
                vec![vec![0, 3], vec![6, 4], vec![8, 5]],
                3,
                2,
            ),
            0
        );
    }
}
