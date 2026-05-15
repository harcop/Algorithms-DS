/// LeetCode #773 - Sliding Puzzle
use std::collections::{HashSet, VecDeque};

fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
    let start: String = board
        .iter()
        .flat_map(|r| r.iter().map(|x| (b'0' + *x as u8) as char))
        .collect();
    if start == "123450" {
        return 0;
    }
    let moves: [&[usize]; 6] = [
        &[1, 3],
        &[0, 2, 4],
        &[1, 5],
        &[0, 4],
        &[1, 3, 5],
        &[2, 4],
    ];
    let goal = "123450";
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    q.push_back((start.clone(), 0i32));
    seen.insert(start);
    while let Some((cur, d)) = q.pop_front() {
        let z = cur.find('0').unwrap();
        let b = cur.as_bytes();
        for &ni in moves[z] {
            let mut v = b.to_vec();
            v[z] = v[ni];
            v[ni] = b'0';
            let next = String::from_utf8(v).unwrap();
            if next == goal {
                return d + 1;
            }
            if seen.insert(next.clone()) {
                q.push_back((next, d + 1));
            }
        }
    }
    -1
}

fn main() {
    let b = vec![vec![1, 2, 3], vec![4, 0, 5]];
    println!("{}", sliding_puzzle(b));
}

#[cfg(test)]
mod tests {
    use super::sliding_puzzle;

    #[test]
    fn example_one() {
        assert_eq!(sliding_puzzle(vec![vec![1, 2, 3], vec![4, 0, 5]]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(sliding_puzzle(vec![vec![1, 2, 3], vec![5, 4, 0]]), -1);
    }
}
