/// LeetCode #631 - Design Excel Sum Formula
use std::collections::HashMap;

struct Excel {
    height: i32,
    grid: Vec<Vec<i32>>,
    formulas: HashMap<(i32, char), Vec<String>>,
}

impl Excel {
    fn new(height: i32, width: char) -> Self {
        let cols = (width as u8 - b'A' + 1) as usize;
        Excel {
            height,
            grid: vec![vec![0; cols]; height as usize],
            formulas: HashMap::new(),
        }
    }

    fn col_idx(c: char) -> usize {
        (c as u8 - b'A') as usize
    }

    fn parse_cell(s: &str) -> (i32, char) {
        let col = s.chars().next().unwrap();
        let row: i32 = s[1..].parse().unwrap();
        (row, col)
    }

    fn raw_get(&self, row: i32, column: char) -> i32 {
        self.grid[(row - 1) as usize][Self::col_idx(column)]
    }

    fn eval_cell(&self, row: i32, column: char) -> i32 {
        if let Some(refs) = self.formulas.get(&(row, column)) {
            let mut sum = 0;
            for r in refs {
                if let Some((a, b)) = r.split_once(':') {
                    let (r1, c1) = Self::parse_cell(a);
                    let (r2, c2) = Self::parse_cell(b);
                    let (rmin, rmax) = (r1.min(r2), r1.max(r2));
                    let (cmin, cmax) = (c1.min(c2), c1.max(c2));
                    for rr in rmin..=rmax {
                        for cc in cmin..=cmax {
                            sum += self.eval_cell(rr, cc);
                        }
                    }
                } else {
                    let (rr, cc) = Self::parse_cell(r);
                    sum += self.eval_cell(rr, cc);
                }
            }
            sum
        } else {
            self.raw_get(row, column)
        }
    }

    fn set(&mut self, row: i32, column: char, val: i32) {
        self.formulas.remove(&(row, column));
        self.grid[(row - 1) as usize][Self::col_idx(column)] = val;
        let _ = self.height;
    }

    fn get(&self, row: i32, column: char) -> i32 {
        self.eval_cell(row, column)
    }

    fn sum(&mut self, row: i32, column: char, numbers: Vec<String>) -> i32 {
        self.formulas.insert((row, column), numbers);
        self.get(row, column)
    }
}

fn main() {
    let mut e = Excel::new(3, 'C');
    e.set(1, 'A', 2);
    println!("{}", e.sum(3, 'C', vec!["A1".into(), "A1:B2".into()]));
}

#[cfg(test)]
mod tests {
    use super::Excel;

    #[test]
    fn example() {
        let mut e = Excel::new(3, 'C');
        e.set(1, 'A', 2);
        assert_eq!(e.sum(3, 'C', vec!["A1".into(), "A1:B2".into()]), 4);
        e.set(2, 'B', 2);
        assert_eq!(e.get(3, 'C'), 6);
    }
}
