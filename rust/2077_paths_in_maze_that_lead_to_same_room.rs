/// LeetCode #2077 - Paths in Maze That Lead to Same Room
use std::collections::HashSet;

fn number_of_paths(n: i32, corridors: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut graph = vec![HashSet::new(); n];
    let mut edges = Vec::with_capacity(corridors.len());

    for corridor in corridors {
        let a = corridor[0] as usize - 1;
        let b = corridor[1] as usize - 1;
        graph[a].insert(b);
        graph[b].insert(a);
        edges.push((a, b));
    }

    let mut count = 0;
    for (a, b) in edges {
        let (small, large) = if graph[a].len() <= graph[b].len() {
            (&graph[a], &graph[b])
        } else {
            (&graph[b], &graph[a])
        };
        count += small.iter().filter(|&&node| large.contains(&node)).count() as i32;
    }

    count / 3
}

fn main() {
    println!(
        "{}",
        number_of_paths(
            5,
            vec![
                vec![1, 2],
                vec![5, 2],
                vec![4, 1],
                vec![2, 4],
                vec![3, 1],
                vec![3, 4],
            ],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::number_of_paths;

    #[test]
    fn example_one() {
        assert_eq!(
            number_of_paths(
                5,
                vec![
                    vec![1, 2],
                    vec![5, 2],
                    vec![4, 1],
                    vec![2, 4],
                    vec![3, 1],
                    vec![3, 4],
                ],
            ),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_paths(4, vec![vec![1, 2], vec![3, 4]]), 0);
    }
}
