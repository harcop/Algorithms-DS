/// LeetCode #1059 - All Paths from Source Lead to Destination
fn leads_to_destination(n: i32, edges: Vec<Vec<i32>>) -> bool {
    let n = n as usize;
    let dest = n - 1;
    let mut out = vec![Vec::new(); n];
    for e in edges {
        out[e[0] as usize].push(e[1] as usize);
    }
    if !out[dest].is_empty() {
        return false;
    }
    let mut visiting = vec![false; n];
    fn dfs(u: usize, dest: usize, out: &Vec<Vec<usize>>, visiting: &mut Vec<bool>) -> bool {
        if u == dest {
            return true;
        }
        if out[u].is_empty() {
            return false;
        }
        if visiting[u] {
            return false;
        }
        visiting[u] = true;
        let ok = out[u].iter().all(|&v| dfs(v, dest, out, visiting));
        visiting[u] = false;
        ok
    }
    dfs(0, dest, &out, &mut visiting)
}

fn main() {
    println!("{}", leads_to_destination(3, vec![vec![0, 1], vec![0, 2]]));
}

#[cfg(test)]
mod tests {
    use super::leads_to_destination;

    #[test]
    fn example_one() {
        assert!(!leads_to_destination(3, vec![vec![0, 1], vec![0, 2]]));
    }

    #[test]
    fn example_two() {
        assert!(leads_to_destination(4, vec![vec![0, 1], vec![0, 3], vec![1, 2], vec![2, 3]]));
    }
}
