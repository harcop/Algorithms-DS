/// LeetCode #1423 - Maximum Points You Can Obtain From Cards
fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    let n = card_points.len();
    let k = k as usize;
    let window = n - k;
    let mut window_sum: i32 = card_points[..window].iter().sum();
    let total: i32 = card_points.iter().sum();
    let mut min_window = window_sum;
    for i in window..n {
        window_sum += card_points[i] - card_points[i - window];
        min_window = min_window.min(window_sum);
    }
    total - min_window
}

fn main() {
    println!("{}", max_score(vec![1, 2, 3, 4, 5, 6, 1], 3));
}

#[cfg(test)]
mod tests {
    use super::max_score;

    #[test]
    fn example_one() {
        assert_eq!(max_score(vec![1, 2, 3, 4, 5, 6, 1], 3), 12);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_score(vec![2, 2, 2], 2), 4);
    }
}

