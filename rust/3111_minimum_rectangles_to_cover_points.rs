/// LeetCode #3111 - Minimum Rectangles to Cover Points
fn min_rectangles_to_cover_points(mut points: Vec<Vec<i32>>, w: i32) -> i32 {
    points.sort_by_key(|p| p[0]);
    let mut ans = 0;
    let mut x1 = -1;
    for p in points {
        let x = p[0];
        if x > x1 {
            ans += 1;
            x1 = x + w;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        min_rectangles_to_cover_points(
            vec![
                vec![2, 1],
                vec![1, 0],
                vec![1, 4],
                vec![1, 8],
                vec![3, 5],
                vec![4, 6]
            ],
            1
        )
    );
}

#[cfg(test)]
mod tests {
    use super::min_rectangles_to_cover_points;

    #[test]
    fn example1() {
        assert_eq!(
            min_rectangles_to_cover_points(
                vec![
                    vec![2, 1],
                    vec![1, 0],
                    vec![1, 4],
                    vec![1, 8],
                    vec![3, 5],
                    vec![4, 6]
                ],
                1
            ),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_rectangles_to_cover_points(
                vec![
                    vec![0, 0],
                    vec![1, 1],
                    vec![2, 2],
                    vec![3, 3],
                    vec![4, 4],
                    vec![5, 5],
                    vec![6, 6]
                ],
                2
            ),
            3
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            min_rectangles_to_cover_points(vec![vec![2, 3], vec![1, 2]], 0),
            2
        );
    }
}
