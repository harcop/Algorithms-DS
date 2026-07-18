/// LeetCode #2477 - Minimum Fuel Cost to Report to the Capital
fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
    let n = roads.len() + 1;
    let mut tree = vec![Vec::new(); n];
    for road in roads {
        let u = road[0] as usize;
        let v = road[1] as usize;
        tree[u].push(v);
        tree[v].push(u);
    }

    let mut answer = 0i64;
    fn dfs(tree: &[Vec<usize>], node: usize, parent: usize, seats: i64, answer: &mut i64) -> i64 {
        let mut people = 1i64;
        for &next in &tree[node] {
            if next != parent {
                people += dfs(tree, next, node, seats, answer);
            }
        }
        if node > 0 {
            *answer += (people + seats - 1) / seats;
        }
        people
    }

    dfs(&tree, 0, usize::MAX, seats as i64, &mut answer);
    answer
}

fn main() {
    println!(
        "{}",
        minimum_fuel_cost(vec![vec![0, 1], vec![0, 2], vec![0, 3]], 5)
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_fuel_cost;

    #[test]
    fn example_one() {
        assert_eq!(
            minimum_fuel_cost(vec![vec![0, 1], vec![0, 2], vec![0, 3]], 5),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            minimum_fuel_cost(
                vec![
                    vec![3, 1],
                    vec![3, 2],
                    vec![1, 0],
                    vec![0, 4],
                    vec![0, 5],
                    vec![4, 6]
                ],
                2
            ),
            7
        );
    }

    #[test]
    fn single_city() {
        assert_eq!(minimum_fuel_cost(vec![], 1), 0);
    }
}
