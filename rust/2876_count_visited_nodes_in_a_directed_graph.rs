/// LeetCode #2876 - Count Visited Nodes in a Directed Graph
fn count_visited_nodes(edges: Vec<i32>) -> Vec<i32> {
    let n = edges.len();
    let mut ans = vec![0; n];
    let mut in_degrees = vec![0; n];
    let mut seen = vec![false; n];
    let mut stack = Vec::new();

    for &v in &edges {
        in_degrees[v as usize] += 1;
    }

    let mut queue: std::collections::VecDeque<_> = (0..n)
        .filter(|&i| in_degrees[i] == 0)
        .collect();

    while let Some(u) = queue.pop_front() {
        let next = edges[u] as usize;
        in_degrees[next] -= 1;
        if in_degrees[next] == 0 {
            queue.push_back(next);
        }
        stack.push(u);
        seen[u] = true;
    }

    for i in 0..n {
        if !seen[i] {
            fill_cycle(&edges, i, &mut seen, &mut ans);
        }
    }

    while let Some(u) = stack.pop() {
        ans[u] = ans[edges[u] as usize] + 1;
    }
    ans
}

fn fill_cycle(edges: &[i32], start: usize, seen: &mut [bool], ans: &mut [i32]) {
    let mut cycle_length = 0;
    let mut u = start;
    while !seen[u] {
        cycle_length += 1;
        seen[u] = true;
        u = edges[u] as usize;
    }
    ans[start] = cycle_length;
    u = edges[start] as usize;
    while u != start {
        ans[u] = cycle_length;
        u = edges[u] as usize;
    }
}

fn main() {
    println!("{:?}", count_visited_nodes(vec![1, 2, 0, 0]));
}

#[cfg(test)]
mod tests {
    use super::count_visited_nodes;

    #[test]
    fn example_one() {
        assert_eq!(count_visited_nodes(vec![1, 2, 0, 0]), vec![3, 3, 3, 4]);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            count_visited_nodes(vec![1, 2, 3, 4, 0]),
            vec![5, 5, 5, 5, 5]
        );
    }
}
