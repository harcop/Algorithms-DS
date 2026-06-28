/// LeetCode #2140 - Most Points Obtaining from Cards
fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    let n = card_points.len();
    let k = k as usize;
    let total: i32 = card_points.iter().sum();
    if k == n {
        return total;
    }

    let window = n - k;
    let mut cur: i32 = card_points[..window].iter().sum();
    let mut min_sum = cur;
    for i in window..n {
        cur += card_points[i] - card_points[i - window];
        min_sum = min_sum.min(cur);
    }

    total - min_sum
}

fn main() {
    println!("{}", max_score(vec![1, 2, 3, 4, 5], 3));
}

#[cfg(test)]
mod tests {
    use super::max_score;

    #[test]
    fn example_one() {
        assert_eq!(max_score(vec![1, 2, 3, 4, 5], 3), 12);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_score(vec![2, 2, 2], 2), 4);
    }

    #[test]
    fn example_three() {
        assert_eq!(max_score(vec![9, 7, 7, 9, 7, 7, 9], 7), 55);
    }
}
