/// LeetCode #2360 - Longest Cycle in a Graph
fn longest_cycle(edges: Vec<i32>) -> i32 {
    let n = edges.len();
    let mut vis = vec![false; n];
    let mut ans = -1;
    for i in 0..n {
        if vis[i] {
            continue;
        }
        let mut j = i as i32;
        let mut cycle = Vec::new();
        while j != -1 && !vis[j as usize] {
            vis[j as usize] = true;
            cycle.push(j);
            j = edges[j as usize];
        }
        if j == -1 {
            continue;
        }
        for k in 0..cycle.len() {
            if cycle[k] == j {
                ans = ans.max((cycle.len() - k) as i32);
                break;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", longest_cycle(vec![3, 3, 4, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::longest_cycle;

    #[test]
    fn example_one() {
        assert_eq!(longest_cycle(vec![3, 3, 4, 2, 3]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_cycle(vec![2, -1, 3, 1]), -1);
    }
}
