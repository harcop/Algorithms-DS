/// LeetCode #452 - Minimum Number of Arrows to Burst Balloons
fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
    if points.is_empty() {
        return 0;
    }
    points.sort_by_key(|p| p[1]);
    let mut arrows = 1;
    let mut end = points[0][1];
    for p in points.into_iter().skip(1) {
        if p[0] > end {
            arrows += 1;
            end = p[1];
        } else {
            end = end.min(p[1]);
        }
    }
    arrows
}

fn main() {
    println!(
        "{}",
        find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]])
    );
}

#[cfg(test)]
mod tests {
    use super::find_min_arrow_shots;

    #[test]
    fn example_one() {
        assert_eq!(
            find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]),
            2
        );
    }
}
