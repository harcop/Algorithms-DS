/// LeetCode #403 - Frog Jump (DP on positions)
use std::collections::{HashMap, HashSet};

fn can_cross(stones: Vec<i32>) -> bool {
    if stones.is_empty() {
        return false;
    }
    let mut set: HashSet<i32> = stones.iter().copied().collect();
    let mut dp: HashMap<(i32, i32), bool> = HashMap::new();
    fn dfs(
        pos: i32,
        jump: i32,
        last: i32,
        stones: &HashSet<i32>,
        dp: &mut HashMap<(i32, i32), bool>,
    ) -> bool {
        if pos == last {
            return true;
        }
        if let Some(&v) = dp.get(&(pos, jump)) {
            return v;
        }
        let mut ok = false;
        for dj in [jump - 1, jump, jump + 1] {
            if dj <= 0 {
                continue;
            }
            let np = pos + dj;
            if stones.contains(&np) && dfs(np, dj, last, stones, dp) {
                ok = true;
                break;
            }
        }
        dp.insert((pos, jump), ok);
        ok
    }
    let last = *stones.last().unwrap();
    dfs(stones[0], 0, last, &set, &mut dp)
}

fn main() {
    println!("{}", can_cross(vec![0, 1, 3, 5, 6, 8, 12, 17]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lc() {
        assert!(can_cross(vec![0, 1, 3, 5, 6, 8, 12, 17]));
        assert!(!can_cross(vec![0, 1, 2, 3, 4, 8, 9, 11]));
    }
}
