/// LeetCode #210 - Course Schedule II
use std::collections::VecDeque;

fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let n = num_courses as usize;
    let mut g: Vec<Vec<usize>> = vec![vec![]; n];
    let mut indeg = vec![0; n];
    for e in prerequisites {
        let a = e[1] as usize;
        let b = e[0] as usize;
        g[a].push(b);
        indeg[b] += 1;
    }
    let mut q = VecDeque::new();
    for i in 0..n {
        if indeg[i] == 0 {
            q.push_back(i);
        }
    }
    let mut out = Vec::with_capacity(n);
    while let Some(u) = q.pop_front() {
        out.push(u as i32);
        for &v in &g[u] {
            indeg[v] -= 1;
            if indeg[v] == 0 {
                q.push_back(v);
            }
        }
    }
    if out.len() == n {
        out
    } else {
        vec![]
    }
}

fn main() {
    println!("{:?}", find_order(2, vec![vec![1, 0]]));
}

#[cfg(test)]
mod tests {
    use super::find_order;

    #[test]
    fn example_one() {
        assert_eq!(find_order(2, vec![vec![1, 0]]), vec![0, 1]);
    }

    #[test]
    fn example_two() {
        assert!(find_order(1, vec![]).is_empty() == false);
        assert_eq!(find_order(1, vec![]), vec![0]);
    }

    #[test]
    fn example_three() {
        assert_eq!(find_order(2, vec![vec![1, 0], vec![0, 1]]), vec![]);
    }
}
