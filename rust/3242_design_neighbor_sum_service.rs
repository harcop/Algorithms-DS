/// LeetCode #3242 - Design Neighbor Sum Service
use std::collections::HashMap;

struct NeighborSum {
    grid: Vec<Vec<i32>>,
    pos: HashMap<i32, (usize, usize)>,
}

impl NeighborSum {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        let mut pos = HashMap::new();
        for (i, row) in grid.iter().enumerate() {
            for (j, &x) in row.iter().enumerate() {
                pos.insert(x, (i, j));
            }
        }
        NeighborSum { grid, pos }
    }

    fn cal(&self, value: i32, dirs: &[(i32, i32)]) -> i32 {
        let &(i, j) = self.pos.get(&value).unwrap();
        let m = self.grid.len() as i32;
        let n = self.grid[0].len() as i32;
        let mut s = 0;
        for &(di, dj) in dirs {
            let x = i as i32 + di;
            let y = j as i32 + dj;
            if x >= 0 && x < m && y >= 0 && y < n {
                s += self.grid[x as usize][y as usize];
            }
        }
        s
    }

    fn adjacent_sum(&self, value: i32) -> i32 {
        self.cal(value, &[(-1, 0), (0, 1), (1, 0), (0, -1)])
    }

    fn diagonal_sum(&self, value: i32) -> i32 {
        self.cal(value, &[(-1, 1), (1, 1), (1, -1), (-1, -1)])
    }
}

fn main() {
    let obj = NeighborSum::new(vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]);
    println!("{}", obj.adjacent_sum(1));
}

#[cfg(test)]
mod tests {
    use super::NeighborSum;

    #[test]
    fn example1() {
        let obj = NeighborSum::new(vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]);
        assert_eq!(obj.adjacent_sum(1), 6);
        assert_eq!(obj.adjacent_sum(4), 16);
        assert_eq!(obj.diagonal_sum(4), 16);
        assert_eq!(obj.diagonal_sum(8), 4);
    }

    #[test]
    fn example2() {
        let obj = NeighborSum::new(vec![
            vec![1, 2, 0, 3],
            vec![4, 7, 15, 6],
            vec![8, 9, 10, 11],
            vec![12, 13, 14, 5],
        ]);
        assert_eq!(obj.adjacent_sum(15), 23);
        assert_eq!(obj.diagonal_sum(9), 45);
    }
}
