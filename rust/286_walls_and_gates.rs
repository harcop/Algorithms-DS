/// LeetCode #286 - Walls and Gates
use std::collections::VecDeque;

fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
    let m = rooms.len();
    if m == 0 {
        return;
    }
    let n = rooms[0].len();
    let mut q = VecDeque::new();
    for i in 0..m {
        for j in 0..n {
            if rooms[i][j] == 0 {
                q.push_back((i, j));
            }
        }
    }
    while let Some((i, j)) = q.pop_front() {
        let d = rooms[i][j] + 1;
        for (di, dj) in [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)] {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 {
                let ni = ni as usize;
                let nj = nj as usize;
                if rooms[ni][nj] == i32::MAX {
                    rooms[ni][nj] = d;
                    q.push_back((ni, nj));
                }
            }
        }
    }
}

fn main() {
    let mut r = vec![vec![i32::MAX, -1, 0, i32::MAX]];
    walls_and_gates(&mut r);
    println!("{:?}", r);
}

#[cfg(test)]
mod tests {
    use super::walls_and_gates;

    #[test]
    fn example_one() {
        let mut rooms = vec![
            vec![i32::MAX, -1, 0, i32::MAX],
            vec![i32::MAX, i32::MAX, i32::MAX, -1],
            vec![i32::MAX, -1, i32::MAX, -1],
            vec![0, -1, i32::MAX, i32::MAX],
        ];
        walls_and_gates(&mut rooms);
        assert_eq!(
            rooms,
            vec![
                vec![3, -1, 0, 1],
                vec![2, 2, 1, -1],
                vec![1, -1, 2, -1],
                vec![0, -1, 3, 4],
            ]
        );
    }
}
