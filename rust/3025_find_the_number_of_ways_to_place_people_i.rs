/// LeetCode #3025 - Find the Number of Ways to Place People I
fn count_ways(points: Vec<Vec<i32>>) -> i64 {
    let mut pts: Vec<(i32, i32)> = points.iter().map(|p| (p[0], p[1])).collect();
    pts.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));

    let mut ans = 0i64;
    for i in 0..pts.len() {
        let y1 = pts[i].1;
        let mut max_y = i32::MIN;
        for j in (i + 1)..pts.len() {
            let y2 = pts[j].1;
            if max_y < y2 && y2 <= y1 {
                ans += 1;
            }
            max_y = max_y.max(y2);
        }
    }
    ans
}

fn main() {
    let points = vec![vec![6, 2], vec![4, 4], vec![2, 6]];
    println!("{}", count_ways(points));
}

#[cfg(test)]
mod tests {
    use super::count_ways;

    #[test]
    fn example1() {
        assert_eq!(
            count_ways(vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
            0
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            count_ways(vec![vec![6, 2], vec![4, 4], vec![2, 6]]),
            2
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            count_ways(vec![vec![3, 1], vec![1, 3], vec![1, 1]]),
            2
        );
    }
}
