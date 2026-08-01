/// LeetCode #2866 - Beautiful Towers II
fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
    let n = max_heights.len();
    let mut left_sum = vec![0_i64; n];
    let mut right_sum = vec![0_i64; n];
    let mut stack = Vec::new();

    for i in 0..n {
        while stack
            .last()
            .is_some_and(|&j| max_heights[j] >= max_heights[i])
        {
            stack.pop();
        }
        left_sum[i] = if let Some(&j) = stack.last() {
            left_sum[j] + (i - j) as i64 * max_heights[i] as i64
        } else {
            (i + 1) as i64 * max_heights[i] as i64
        };
        stack.push(i);
    }

    stack.clear();
    for i in (0..n).rev() {
        while stack
            .last()
            .is_some_and(|&j| max_heights[j] > max_heights[i])
        {
            stack.pop();
        }
        right_sum[i] = if let Some(&j) = stack.last() {
            right_sum[j] + (j - i) as i64 * max_heights[i] as i64
        } else {
            (n - i) as i64 * max_heights[i] as i64
        };
        stack.push(i);
    }

    (0..n)
        .map(|i| left_sum[i] + right_sum[i] - max_heights[i] as i64)
        .max()
        .unwrap_or(0)
}

fn main() {
    println!("{}", maximum_sum_of_heights(vec![5, 3, 4, 1, 1]));
}

#[cfg(test)]
mod tests {
    use super::maximum_sum_of_heights;

    #[test]
    fn example_one() {
        assert_eq!(maximum_sum_of_heights(vec![5, 3, 4, 1, 1]), 13);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_sum_of_heights(vec![6, 5, 3, 9, 2, 7]), 22);
    }

    #[test]
    fn example_three() {
        assert_eq!(maximum_sum_of_heights(vec![3, 2, 5, 5, 2, 3]), 18);
    }
}
