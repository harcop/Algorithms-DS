/// LeetCode #886 - Possible Bipartition
fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
    let n = n as usize;
    let mut adj = vec![vec![]; n + 1];
    for e in dislikes {
        let a = e[0] as usize;
        let b = e[1] as usize;
        adj[a].push(b);
        adj[b].push(a);
    }
    let mut color = vec![0i8; n + 1];
    for start in 1..=n {
        if color[start] != 0 {
            continue;
        }
        color[start] = 1;
        let mut stack = vec![start];
        while let Some(u) = stack.pop() {
            for &v in &adj[u] {
                if color[v] == 0 {
                    color[v] = -color[u];
                    stack.push(v);
                } else if color[v] == color[u] {
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    println!("{}", possible_bipartition(4, vec![vec![1, 2], vec![1, 3], vec![2, 4]]));
}

#[cfg(test)]
mod tests {
    use super::possible_bipartition;

    #[test]
    fn example_one() {
        assert!(possible_bipartition(4, vec![vec![1, 2], vec![1, 3], vec![2, 4]]));
    }
}
