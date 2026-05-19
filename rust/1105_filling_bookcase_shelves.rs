/// LeetCode #1105 - Filling Bookcase Shelves
fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
    let n = books.len();
    let mut dp = vec![i32::MAX; n + 1];
    dp[0] = 0;
    for i in 1..=n {
        let mut w = 0;
        let mut h = 0;
        for j in (0..i).rev() {
            w += books[j][0];
            h = h.max(books[j][1]);
            if w > shelf_width {
                break;
            }
            dp[i] = dp[i].min(dp[j] + h);
        }
    }
    dp[n]
}

fn main() {
    println!(
        "{}",
        min_height_shelves(vec![vec![1, 1], vec![2, 3], vec![2, 3], vec![1, 1], vec![1, 1], vec![1, 1], vec![1, 2]], 4)
    );
}

#[cfg(test)]
mod tests {
    use super::min_height_shelves;

    #[test]
    fn example_one() {
        assert_eq!(
            min_height_shelves(vec![vec![1, 1], vec![2, 3], vec![2, 3], vec![1, 1], vec![1, 1], vec![1, 1], vec![1, 2]], 4),
            6
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(min_height_shelves(vec![vec![1, 2], vec![1, 3]], 5), 3);
    }
}
