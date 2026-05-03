/// LeetCode #207 - Course Schedule
use std::collections::VecDeque;

fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
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
    let mut done = 0;
    while let Some(u) = q.pop_front() {
        done += 1;
        for &v in &g[u] {
            indeg[v] -= 1;
            if indeg[v] == 0 {
                q.push_back(v);
            }
        }
    }
    done == n
}

fn main() {
    println!(
        "{}",
        can_finish(2, vec![vec![1, 0]])
    );
}

#[cfg(test)]
mod tests {
    use super::can_finish;

    #[test]
    fn example_one() {
        assert!(can_finish(2, vec![vec![1, 0]]));
    }

    #[test]
    fn example_two() {
        assert!(!can_finish(2, vec![vec![1, 0], vec![0, 1]]));
    }
}
