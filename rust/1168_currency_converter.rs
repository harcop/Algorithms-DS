/// LeetCode #1168 - Optimize Water Distribution in a Village
fn min_cost_to_supply_water(n: i32, wells: Vec<i32>, pipes: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut edges: Vec<(i32, usize, usize)> = Vec::new();
    for (i, &w) in wells.iter().enumerate() {
        edges.push((w, 0, i + 1));
    }
    for p in pipes {
        edges.push((p[2], p[0] as usize, p[1] as usize));
    }
    edges.sort_by_key(|e| e.0);
    let mut parent: Vec<usize> = (0..=n).collect();
    fn find(parent: &mut [usize], x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }
    let mut cost = 0;
    let mut used = 0;
    for (c, a, b) in edges {
        let pa = find(&mut parent, a);
        let pb = find(&mut parent, b);
        if pa != pb {
            parent[pa] = pb;
            cost += c;
            used += 1;
            if used == n {
                break;
            }
        }
    }
    cost
}

fn main() {
    println!(
        "{}",
        min_cost_to_supply_water(3, vec![1, 2, 2], vec![vec![1, 2, 1], vec![2, 3, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::min_cost_to_supply_water;

    #[test]
    fn example_one() {
        assert_eq!(
            min_cost_to_supply_water(3, vec![1, 2, 2], vec![vec![1, 2, 1], vec![2, 3, 1]]),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            min_cost_to_supply_water(2, vec![1, 1], vec![vec![1, 2, 1], vec![1, 2, 2]]),
            2
        );
    }
}
