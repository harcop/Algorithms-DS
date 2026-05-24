/// LeetCode #1328 - Check If It Is a Straight Line
fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
    let (x0, y0) = (coordinates[0][0], coordinates[0][1]);
    let (x1, y1) = (coordinates[1][0], coordinates[1][1]);
    let dx = x1 - x0;
    let dy = y1 - y0;
    for p in coordinates.iter().skip(2) {
        if (p[0] - x0) * dy != (p[1] - y0) * dx {
            return false;
        }
    }
    true
}

fn main() {
    println!("{}", check_straight_line(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7]]));
}

#[cfg(test)]
mod tests {
    use super::check_straight_line;

    #[test]
    fn example_one() {
        assert!(check_straight_line(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7]]));
    }

    #[test]
    fn example_two() {
        assert!(!check_straight_line(vec![vec![1, 1], vec![2, 2], vec![3, 4], vec![4, 5], vec![5, 6], vec![7, 7]]));
    }
}
