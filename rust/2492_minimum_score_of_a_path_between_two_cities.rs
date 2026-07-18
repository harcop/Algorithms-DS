/// LeetCode #2492 - Minimum Score of a Path Between Two Cities
use std::collections::VecDeque;

fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut graph = vec![Vec::new(); n + 1];
    for road in roads {
        let u = road[0] as usize;
        let v = road[1] as usize;
        let distance = road[2];
        graph[u].push((v, distance));
        graph[v].push((u, distance));
    }

    let mut answer = i32::MAX;
    let mut seen = vec![false; n + 1];
    let mut queue = VecDeque::from([1usize]);
    seen[1] = true;

    while let Some(u) = queue.pop_front() {
        for &(v, distance) in &graph[u] {
            answer = answer.min(distance);
            if !seen[v] {
                seen[v] = true;
                queue.push_back(v);
            }
        }
    }

    answer
}

fn main() {
    println!(
        "{}",
        min_score(
            4,
            vec![vec![1, 2, 9], vec![2, 3, 6], vec![2, 4, 5], vec![1, 4, 7]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::min_score;

    #[test]
    fn example_one() {
        assert_eq!(
            min_score(
                4,
                vec![vec![1, 2, 9], vec![2, 3, 6], vec![2, 4, 5], vec![1, 4, 7]]
            ),
            5
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            min_score(3, vec![vec![1, 2, 2], vec![1, 3, 4], vec![2, 3, 1]]),
            1
        );
    }
}
