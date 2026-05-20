/// LeetCode #1136 - Parallel Courses
use std::collections::VecDeque;

fn minimum_semesters(n: i32, relations: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut indeg = vec![0usize; n + 1];
    let mut adj = vec![Vec::new(); n + 1];
    for r in relations {
        let u = r[0] as usize;
        let v = r[1] as usize;
        adj[u].push(v);
        indeg[v] += 1;
    }
    let mut q = VecDeque::new();
    for i in 1..=n {
        if indeg[i] == 0 {
            q.push_back(i);
        }
    }
    let mut semesters = 0i32;
    let mut taken = 0usize;
    while !q.is_empty() {
        semesters += 1;
        let sz = q.len();
        for _ in 0..sz {
            let u = q.pop_front().unwrap();
            taken += 1;
            for &v in &adj[u] {
                indeg[v] -= 1;
                if indeg[v] == 0 {
                    q.push_back(v);
                }
            }
        }
    }
    if taken == n { semesters } else { -1 }
}

fn main() {
    println!("{}", minimum_semesters(3, vec![vec![1, 3], vec![2, 3]]));
}

#[cfg(test)]
mod tests {
    use super::minimum_semesters;

    #[test]
    fn example_one() {
        assert_eq!(minimum_semesters(3, vec![vec![1, 3], vec![2, 3]]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_semesters(3, vec![vec![1, 2], vec![2, 3], vec![3, 1]]), -1);
    }
}
