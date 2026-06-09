/// LeetCode #1791 - Find Center of Star Graph
fn find_center(edges: Vec<Vec<i32>>) -> i32 {
    if edges[1].contains(&edges[0][0]) {
        edges[0][0]
    } else {
        edges[0][1]
    }
}

fn main() {
    println!(
        "{}",
        find_center(vec![vec![1, 2], vec![2, 3], vec![4, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::find_center;

    #[test]
    fn example_one() {
        assert_eq!(
            find_center(vec![vec![1, 2], vec![2, 3], vec![4, 2]]),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_center(vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]]),
            1
        );
    }
}
