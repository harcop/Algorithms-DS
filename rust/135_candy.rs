/// LeetCode #135 - Candy
fn candy(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    let mut left = vec![1; n];
    for i in 1..n {
        if ratings[i] > ratings[i - 1] {
            left[i] = left[i - 1] + 1;
        }
    }
    let mut right = vec![1; n];
    for i in (0..n - 1).rev() {
        if ratings[i] > ratings[i + 1] {
            right[i] = right[i + 1] + 1;
        }
    }
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| *a.max(b))
        .sum()
}

fn main() {
    println!("{}", candy(vec![1, 0, 2]));
}

#[cfg(test)]
mod tests {
    use super::candy;

    #[test]
    fn example_one() {
        assert_eq!(candy(vec![1, 0, 2]), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(candy(vec![1, 2, 2]), 4);
    }
}
