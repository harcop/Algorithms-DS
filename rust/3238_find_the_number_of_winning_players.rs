/// LeetCode #3238 - Find the Number of Winning Players
use std::collections::HashSet;

fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut cnt = vec![[0i32; 11]; n];
    let mut s = HashSet::new();
    for p in pick {
        let x = p[0] as usize;
        let y = p[1] as usize;
        cnt[x][y] += 1;
        if cnt[x][y] > x as i32 {
            s.insert(x);
        }
    }
    s.len() as i32
}

fn main() {
    println!(
        "{}",
        winning_player_count(4, vec![vec![0, 0], vec![1, 0], vec![1, 0], vec![2, 1], vec![2, 1], vec![2, 0]])
    );
}

#[cfg(test)]
mod tests {
    use super::winning_player_count;

    #[test]
    fn example1() {
        assert_eq!(
            winning_player_count(
                4,
                vec![
                    vec![0, 0],
                    vec![1, 0],
                    vec![1, 0],
                    vec![2, 1],
                    vec![2, 1],
                    vec![2, 0]
                ]
            ),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            winning_player_count(5, vec![vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4]]),
            0
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            winning_player_count(5, vec![vec![1, 1], vec![2, 4], vec![2, 4], vec![2, 4]]),
            1
        );
    }
}
