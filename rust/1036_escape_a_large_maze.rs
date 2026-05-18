/// LeetCode #1036 - Escape a Large Maze
use std::collections::{HashSet, VecDeque};

fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
    let blocked: HashSet<(i32, i32)> = blocked.into_iter().map(|v| (v[0], v[1])).collect();
    let target = (target[0], target[1]);
    let limit = blocked.len() * blocked.len();
    can_reach(&blocked, (source[0], source[1]), target, limit)
        || can_reach(&blocked, (source[0] + 1, source[1]), target, limit)
}

fn can_reach(blocked: &HashSet<(i32, i32)>, start: (i32, i32), target: (i32, i32), limit: usize) -> bool {
    if blocked.contains(&start) {
        return false;
    }
    if start == target {
        return true;
    }
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut seen = HashSet::new();
    let mut q = VecDeque::new();
    seen.insert(start);
    q.push_back(start);
    while let Some((x, y)) = q.pop_front() {
        if (x, y) == target {
            return true;
        }
        if seen.len() > limit {
            return true;
        }
        for (dx, dy) in dirs {
            let p = (x + dx, y + dy);
            if blocked.contains(&p) || !seen.insert(p) {
                continue;
            }
            q.push_back(p);
        }
    }
    false
}

fn main() {
    println!("{}", is_escape_possible(vec![vec![0, 1], vec![1, 0]], vec![0, 0], vec![0, 2]));
}

#[cfg(test)]
mod tests {
    use super::is_escape_possible;

    #[test]
    fn example_one() {
        assert!(is_escape_possible(vec![vec![0, 1], vec![1, 0]], vec![0, 0], vec![0, 2]));
    }

}
