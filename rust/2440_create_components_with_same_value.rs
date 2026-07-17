/// LeetCode #2440 - Create Components With Same Value
fn component_value(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
    let n = nums.len();
    let mut graph = vec![Vec::new(); n];
    for edge in edges {
        let a = edge[0] as usize;
        let b = edge[1] as usize;
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut parent = vec![n; n];
    let mut order = vec![0];
    let mut index = 0;
    while index < order.len() {
        let node = order[index];
        for &next in &graph[node] {
            if next != parent[node] {
                parent[next] = node;
                order.push(next);
            }
        }
        index += 1;
    }

    let total: i32 = nums.iter().sum();
    for components in (1..=n).rev() {
        if total % components as i32 != 0 {
            continue;
        }

        let target = total / components as i32;
        let mut sums = nums.clone();
        let mut valid = true;

        for &node in order.iter().rev() {
            if sums[node] > target {
                valid = false;
                break;
            }
            if sums[node] < target {
                if parent[node] == n {
                    valid = false;
                    break;
                }
                sums[parent[node]] += sums[node];
            }
        }

        if valid {
            return components as i32 - 1;
        }
    }

    0
}

fn main() {
    println!(
        "{}",
        component_value(
            vec![6, 2, 2, 2, 6],
            vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::component_value;

    #[test]
    fn example_one() {
        assert_eq!(
            component_value(
                vec![6, 2, 2, 2, 6],
                vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]]
            ),
            2
        );
    }

    #[test]
    fn cannot_remove_an_edge() {
        assert_eq!(
            component_value(vec![2, 4, 3], vec![vec![0, 1], vec![1, 2]]),
            0
        );
    }
}
