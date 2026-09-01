/// LeetCode #3547 - Maximum Sum of Edge Values in a Graph
use std::collections::VecDeque;

fn calculate_score(left: i64, right: i64, is_cycle: bool) -> i64 {
    let mut window = VecDeque::from([right, right]);
    let mut score = 0i64;
    let mut value = right - 1;
    while value >= left {
        let window_value = window.pop_front().unwrap();
        score += window_value * value;
        window.push_back(value);
        value -= 1;
    }
    score + window[0] * window[1] * if is_cycle { 1 } else { 0 }
}

fn max_score(n: i32, edges: Vec<Vec<i32>>) -> i64 {
    let n_us = n as usize;
    let mut graph = vec![Vec::new(); n_us];
    for e in &edges {
        graph[e[0] as usize].push(e[1] as usize);
        graph[e[1] as usize].push(e[0] as usize);
    }
    let mut seen = vec![false; n_us];
    let mut cycle_sizes = Vec::new();
    let mut path_sizes = Vec::new();
    for i in 0..n_us {
        if seen[i] {
            continue;
        }
        let mut component = vec![i];
        seen[i] = true;
        let mut idx = 0;
        while idx < component.len() {
            let u = component[idx];
            for &v in &graph[u] {
                if !seen[v] {
                    seen[v] = true;
                    component.push(v);
                }
            }
            idx += 1;
        }
        if component.iter().all(|&u| graph[u].len() == 2) {
            cycle_sizes.push(component.len() as i64);
        } else if component.len() > 1 {
            path_sizes.push(component.len() as i64);
        }
    }
    let mut remaining = n as i64;
    let mut ans = 0i64;
    for sz in cycle_sizes {
        ans += calculate_score(remaining - sz + 1, remaining, true);
        remaining -= sz;
    }
    path_sizes.sort_unstable_by(|a, b| b.cmp(a));
    for sz in path_sizes {
        ans += calculate_score(remaining - sz + 1, remaining, false);
        remaining -= sz;
    }
    ans
}

fn main() {
    println!("{}", max_score(4, vec![vec![0, 1], vec![1, 2], vec![2, 3]]));
}

#[cfg(test)]
mod tests {
    use super::max_score;

    #[test]
    fn example1() {
        assert_eq!(max_score(4, vec![vec![0, 1], vec![1, 2], vec![2, 3]]), 23);
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_score(6, vec![vec![0, 3], vec![4, 5], vec![2, 0], vec![1, 3], vec![2, 4], vec![1, 5]]),
            82
        );
    }
}
