/// LeetCode #2865 - Beautiful Towers I
fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
    let n = max_heights.len();
    let mut answer = 0_i64;

    for peak in 0..n {
        let mut sum = max_heights[peak] as i64;
        let mut height = max_heights[peak];
        for i in (0..peak).rev() {
            height = height.min(max_heights[i]);
            sum += height as i64;
        }

        height = max_heights[peak];
        for &maximum in &max_heights[peak + 1..] {
            height = height.min(maximum);
            sum += height as i64;
        }
        answer = answer.max(sum);
    }
    answer
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
