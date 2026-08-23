/// LeetCode #3391 - Design a 3D Binary Matrix with Efficient Layer Tracking
use std::cmp::Reverse;
use std::collections::BTreeSet;

struct Matrix3D {
    g: Vec<Vec<Vec<i32>>>,
    cnt: Vec<i32>,
    sl: BTreeSet<(Reverse<i32>, Reverse<usize>)>,
}

impl Matrix3D {
    fn new(n: i32) -> Self {
        let n = n as usize;
        Matrix3D {
            g: vec![vec![vec![0; n]; n]; n],
            cnt: vec![0; n],
            sl: BTreeSet::new(),
        }
    }

    fn set_cell(&mut self, x: i32, y: i32, z: i32) {
        let (x, y, z) = (x as usize, y as usize, z as usize);
        if self.g[x][y][z] != 0 {
            return;
        }
        self.g[x][y][z] = 1;
        self.sl.remove(&(Reverse(self.cnt[x]), Reverse(x)));
        self.cnt[x] += 1;
        self.sl.insert((Reverse(self.cnt[x]), Reverse(x)));
    }

    fn unset_cell(&mut self, x: i32, y: i32, z: i32) {
        let (x, y, z) = (x as usize, y as usize, z as usize);
        if self.g[x][y][z] == 0 {
            return;
        }
        self.g[x][y][z] = 0;
        self.sl.remove(&(Reverse(self.cnt[x]), Reverse(x)));
        self.cnt[x] -= 1;
        if self.cnt[x] > 0 {
            self.sl.insert((Reverse(self.cnt[x]), Reverse(x)));
        }
    }

    fn largest_matrix(&self) -> i32 {
        if let Some(&(_, Reverse(x))) = self.sl.iter().next() {
            x as i32
        } else {
            self.g.len() as i32 - 1
        }
    }
}

fn main() {
    let mut m = Matrix3D::new(3);
    m.set_cell(0, 0, 0);
    println!("{}", m.largest_matrix());
}

#[cfg(test)]
mod tests {
    use super::Matrix3D;

    #[test]
    fn example1() {
        let mut m = Matrix3D::new(3);
        m.set_cell(0, 0, 0);
        assert_eq!(m.largest_matrix(), 0);
        m.set_cell(1, 1, 2);
        assert_eq!(m.largest_matrix(), 1);
        m.set_cell(0, 0, 1);
        assert_eq!(m.largest_matrix(), 0);
    }

    #[test]
    fn example2() {
        let mut m = Matrix3D::new(4);
        m.set_cell(2, 1, 1);
        assert_eq!(m.largest_matrix(), 2);
        m.unset_cell(2, 1, 1);
        assert_eq!(m.largest_matrix(), 3);
    }
}
