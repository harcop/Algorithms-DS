/// LeetCode #2350 - Shortest Impossible Sequence of Rolls
use std::collections::HashSet;

fn shortest_sequence(rolls: Vec<i32>, k: i32) -> i32 {
    let mut s = HashSet::new();
    let mut ans = 1;
    for v in rolls {
        s.insert(v);
        if s.len() == k as usize {
            s.clear();
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        shortest_sequence(vec![4, 2, 1, 2, 3, 3, 2, 4, 1], 4)
    );
}

#[cfg(test)]
mod tests {
    use super::shortest_sequence;

    #[test]
    fn example_one() {
        assert_eq!(
            shortest_sequence(vec![4, 2, 1, 2, 3, 3, 2, 4, 1], 4),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(shortest_sequence(vec![1, 1, 2, 2], 2), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(
            shortest_sequence(vec![1, 1, 3, 2, 2, 2, 3, 3], 4),
            1
        );
    }
}
