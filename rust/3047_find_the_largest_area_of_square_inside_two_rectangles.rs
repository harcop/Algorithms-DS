/// LeetCode #3047 - Find the Largest Area of Square Inside Two Rectangles
fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
    let n = bottom_left.len();
    let mut ans = 0i64;

    for i in 0..n {
        for j in (i + 1)..n {
            let w = top_right[i][0]
                .min(top_right[j][0])
                .saturating_sub(bottom_left[i][0].max(bottom_left[j][0]));
            let h = top_right[i][1]
                .min(top_right[j][1])
                .saturating_sub(bottom_left[i][1].max(bottom_left[j][1]));
            let side = w.min(h) as i64;
            if side > 0 {
                ans = ans.max(side * side);
            }
        }
    }

    ans
}

fn main() {
    println!(
        "{}",
        largest_square_area(
            vec![vec![1, 1], vec![2, 2], vec![3, 1]],
            vec![vec![3, 3], vec![4, 4], vec![6, 6]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::largest_square_area;

    #[test]
    fn example1() {
        assert_eq!(
            largest_square_area(
                vec![vec![1, 1], vec![2, 2], vec![3, 1]],
                vec![vec![3, 3], vec![4, 4], vec![6, 6]]
            ),
            1
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            largest_square_area(
                vec![vec![1, 1], vec![1, 3], vec![1, 5]],
                vec![vec![5, 5], vec![5, 7], vec![5, 9]]
            ),
            4
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            largest_square_area(
                vec![vec![1, 1], vec![2, 2], vec![1, 2]],
                vec![vec![3, 3], vec![4, 4], vec![3, 4]]
            ),
            1
        );
    }

    #[test]
    fn example4() {
        assert_eq!(
            largest_square_area(
                vec![vec![1, 1], vec![3, 3], vec![3, 1]],
                vec![vec![2, 2], vec![4, 4], vec![4, 2]]
            ),
            0
        );
    }
}
