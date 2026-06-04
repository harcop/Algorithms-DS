/// LeetCode #1725 - Number Of Rectangles That Can Form The Largest Square
fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
    let mut max_len = 0;
    let mut count = 0;
    for r in rectangles {
        let side = r[0].min(r[1]);
        if side > max_len {
            max_len = side;
            count = 1;
        } else if side == max_len {
            count += 1;
        }
    }
    count
}
fn main() {
    println!(
        "{}",
        count_good_rectangles(vec![vec![5, 8], vec![3, 9], vec![5, 3], vec![9, 7]])
    );
}
#[cfg(test)]
mod tests {
    use super::count_good_rectangles;
    #[test]
    fn example_one() {
        assert_eq!(
            count_good_rectangles(vec![vec![5, 8], vec![3, 9], vec![5, 3], vec![9, 7]]),
            1
        );
    }
    #[test]
    fn example_two() {
        assert_eq!(
            count_good_rectangles(vec![vec![2, 3], vec![3, 7], vec![4, 3], vec![3, 7]]),
            3
        );
    }
}
