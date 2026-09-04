/// LeetCode #1135 - Connecting Cities With Minimum Cost
fn minimum_cost(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut edges = connections;
    edges.sort_by_key(|e| e[2]);
    let mut parent: Vec<usize> = (0..=n).collect();
    fn find(parent: &mut [usize], x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }
    let mut cost = 0;
    let mut used = 0;
    for e in edges {
        let a = find(&mut parent, e[0] as usize);
        let b = find(&mut parent, e[1] as usize);
        if a != b {
            parent[a] = b;
            cost += e[2];
            used += 1;
            if used == n - 1 {
                return cost;
            }
        }
    }
    -1
}

fn main() {
    println!(
        "{}",
        minimum_cost(3, vec![vec![1, 2, 5], vec![1, 3, 6], vec![2, 3, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_cost;

    #[test]
    fn example_one() {
        assert_eq!(
            minimum_cost(3, vec![vec![1, 2, 5], vec![1, 3, 6], vec![2, 3, 1]]),
            6
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_cost(4, vec![vec![1, 2, 3], vec![3, 4, 4]]), -1);
    }
}
