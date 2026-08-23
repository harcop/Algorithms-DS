/// LeetCode #3380 - Maximum Area Rectangle With Point Constraints I
fn max_rectangle_area(points: Vec<Vec<i32>>) -> i32 {
    fn check(points: &[Vec<i32>], x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
        let mut cnt = 0;
        for p in points {
            let (x, y) = (p[0], p[1]);
            if x < x1 || x > x2 || y < y1 || y > y2 {
                continue;
            }
            if (x == x1 || x == x2) && (y == y1 || y == y2) {
                cnt += 1;
                continue;
            }
            return false;
        }
        cnt == 4
    }
    let mut ans = -1;
    for i in 0..points.len() {
        for j in 0..i {
            let x1 = points[i][0].min(points[j][0]);
            let y1 = points[i][1].min(points[j][1]);
            let x2 = points[i][0].max(points[j][0]);
            let y2 = points[i][1].max(points[j][1]);
            if check(&points, x1, y1, x2, y2) {
                ans = ans.max((x2 - x1) * (y2 - y1));
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        max_rectangle_area(vec![vec![1, 1], vec![1, 3], vec![3, 1], vec![3, 3]])
    );
}

#[cfg(test)]
mod tests {
    use super::max_rectangle_area;

    #[test]
    fn example1() {
        assert_eq!(
            max_rectangle_area(vec![vec![1, 1], vec![1, 3], vec![3, 1], vec![3, 3]]),
            4
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_rectangle_area(vec![
                vec![1, 1],
                vec![1, 3],
                vec![3, 1],
                vec![3, 3],
                vec![2, 2]
            ]),
            -1
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            max_rectangle_area(vec![
                vec![1, 1],
                vec![1, 3],
                vec![3, 1],
                vec![3, 3],
                vec![1, 2],
                vec![3, 2]
            ]),
            2
        );
    }
}
