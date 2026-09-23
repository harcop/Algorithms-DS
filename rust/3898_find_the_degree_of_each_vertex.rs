/// LeetCode #3898 - Find the Degree of Each Vertex
fn find_degree(adj: Vec<Vec<i32>>) -> Vec<i32> {
    adj.into_iter()
        .map(|row| row.iter().sum())
        .collect()
}

fn main() {
    println!(
        "{:?}",
        find_degree(vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]])
    );
}

#[cfg(test)]
mod tests {
    use super::find_degree;

    #[test]
    fn example1() {
        assert_eq!(
            find_degree(vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]]),
            vec![2, 2, 2]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            find_degree(vec![vec![0, 1, 0], vec![1, 0, 0], vec![0, 0, 0]]),
            vec![1, 1, 0]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(find_degree(vec![vec![0]]), vec![0]);
    }
}
