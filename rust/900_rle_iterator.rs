/// LeetCode #900 - RLE Iterator
struct RLEIterator {
    enc: Vec<i32>,
    i: usize,
    skip: i32,
}

impl RLEIterator {
    fn new(encoding: Vec<i32>) -> Self {
        RLEIterator {
            enc: encoding,
            i: 0,
            skip: 0,
        }
    }

    fn next(&mut self, n: i32) -> i32 {
        let mut need = n;
        while self.i < self.enc.len() {
            let have = self.enc[self.i] - self.skip;
            if have >= need {
                self.skip += need;
                let v = self.enc[self.i + 1];
                if self.skip == self.enc[self.i] {
                    self.i += 2;
                    self.skip = 0;
                }
                return v;
            }
            need -= have;
            self.i += 2;
            self.skip = 0;
        }
        -1
    }
}

fn main() {
    let mut it = RLEIterator::new(vec![3, 8, 0, 9, 2, 5]);
    println!("{}", it.next(2));
}

#[cfg(test)]
mod tests {
    use super::RLEIterator;

    #[test]
    fn example_one() {
        let mut it = RLEIterator::new(vec![3, 8, 0, 9, 2, 5]);
        assert_eq!(it.next(2), 8);
        assert_eq!(it.next(1), 8);
        assert_eq!(it.next(1), 5);
    }
}
