/// LeetCode #573 - Squirrel Simulation
fn min_distance(height: i32, width: i32, tree: Vec<i32>, squirrel: Vec<i32>, nuts: Vec<Vec<i32>>) -> i32 {
    let _ = (height, width);
    fn dist(a: &[i32], b: &[i32]) -> i32 {
        (a[0] - b[0]).abs() + (a[1] - b[1]).abs()
    }
    let mut total = 0;
    let mut best_save = i32::MIN;
    for nut in &nuts {
        let t = dist(&tree, nut);
        total += t * 2;
        best_save = best_save.max(t - dist(&squirrel, nut));
    }
    total - best_save
}

fn main() {
    let tree = vec![2, 2];
    let squirrel = vec![4, 4];
    let nuts = vec![vec![3, 0], vec![2, 5]];
    println!("{}", min_distance(5, 7, tree, squirrel, nuts));
}

#[cfg(test)]
mod tests {
    use super::min_distance;

    #[test]
    fn example_one() {
        let tree = vec![2, 2];
        let squirrel = vec![4, 4];
        let nuts = vec![vec![3, 0], vec![2, 5]];
        assert_eq!(min_distance(5, 7, tree, squirrel, nuts), 12);
    }

    #[test]
    fn example_two() {
        let tree = vec![0, 1];
        let squirrel = vec![2, 3];
        let nuts = vec![vec![0, 0]];
        assert_eq!(min_distance(1, 3, tree, squirrel, nuts), 6);
    }
}
