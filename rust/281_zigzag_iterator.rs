/// LeetCode #281 - Zigzag Iterator
pub struct ZigzagIterator {
    v: [Vec<i32>; 2],
    idx: [usize; 2],
    turn: usize,
}

impl ZigzagIterator {
    fn new(v1: Vec<i32>, v2: Vec<i32>) -> Self {
        ZigzagIterator {
            v: [v1, v2],
            idx: [0, 0],
            turn: 0,
        }
    }

    fn next(&mut self) -> i32 {
        for _ in 0..2 {
            let t = self.turn;
            let i = self.idx[t];
            if i < self.v[t].len() {
                let x = self.v[t][i];
                self.idx[t] += 1;
                self.turn ^= 1;
                return x;
            }
            self.turn ^= 1;
        }
        unreachable!()
    }

    fn has_next(&self) -> bool {
        self.idx[0] < self.v[0].len() || self.idx[1] < self.v[1].len()
    }
}

fn main() {
    let mut z = ZigzagIterator::new(vec![1, 2], vec![3, 4, 5, 6]);
    while z.has_next() {
        print!("{} ", z.next());
    }
}

#[cfg(test)]
mod tests {
    use super::ZigzagIterator;

    #[test]
    fn example() {
        let mut z = ZigzagIterator::new(vec![1, 2], vec![3, 4, 5, 6]);
        let mut out = vec![];
        while z.has_next() {
            out.push(z.next());
        }
        assert_eq!(out, vec![1, 3, 2, 4, 5, 6]);
    }
}
