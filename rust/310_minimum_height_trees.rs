/// LeetCode #310 - Minimum Height Trees
use std::collections::{HashMap, VecDeque};

fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    if n == 1 {
        return vec![0];
    }
    let mut g: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut deg = vec![0; n];
    for e in edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        g.entry(u).or_default().push(v);
        g.entry(v).or_default().push(u);
        deg[u] += 1;
        deg[v] += 1;
    }
    let mut q = VecDeque::new();
    for i in 0..n {
        if deg[i] == 1 {
            q.push_back(i);
        }
    }
    let mut remaining = n;
    while remaining > 2 {
        let sz = q.len();
        for _ in 0..sz {
            let u = q.pop_front().unwrap();
            remaining -= 1;
            if let Some(nbrs) = g.get(&u) {
                for &v in nbrs {
                    deg[v] -= 1;
                    if deg[v] == 1 {
                        q.push_back(v);
                    }
                }
            }
        }
    }
    q.into_iter().map(|x| x as i32).collect()
}

fn main() {
    println!("{:?}", find_min_height_trees(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]]));
}

#[cfg(test)]
mod tests {
    use super::find_min_height_trees;

    #[test]
    fn example_one() {
        let mut v = find_min_height_trees(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]]);
        v.sort();
        assert_eq!(v, vec![1]);
    }

    #[test]
    fn example_two() {
        let mut v = find_min_height_trees(6, vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]]);
        v.sort();
        assert_eq!(v, vec![3, 4]);
    }
}
