/// LeetCode #1654 - Minimum Jumps To Reach Home
use std::collections::HashSet;

fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
    let mut ban: HashSet<i32> = forbidden.into_iter().collect();
    ban.insert(0);
    let mut q = std::collections::VecDeque::from([(0i32, 0i32, 0i32)]);
    let mut seen: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    while let Some((pos, back, steps)) = q.pop_front() {
        if pos == x { return steps; }
        for (np, nback) in [(pos + a, 0), (pos - b, 1)] {
            if np < 0 || np > 6000 || ban.contains(&np) { continue; }
            if nback == 1 && back == 1 { continue; }
            if seen.insert((np, nback)) { q.push_back((np, nback, steps + 1)); }
        }
    }
    -1
}
fn main() { println!("{}", minimum_jumps(vec![14,4,18,1,15], 3, 15, 9)); }
#[cfg(test)]
mod tests {
    use super::minimum_jumps;
    #[test]
    fn example_one() { assert_eq!(minimum_jumps(vec![14,4,18,1,15], 3, 15, 9), 3); }
}