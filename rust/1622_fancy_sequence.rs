/// LeetCode #1622 - Fancy Sequence
const MOD: i64 = 1_000_000_007;

pub struct Fancy {
    vals: Vec<i64>,
    add: i64,
    mul: i64,
}

impl Fancy {
    fn new() -> Self { Fancy { vals: vec![], add: 0, mul: 1 } }
    fn append(&mut self, val: i32) {
        for i in 0..self.vals.len() {
            self.vals[i] = (self.vals[i] * self.mul % MOD + self.add) % MOD;
        }
        self.mul = 1;
        self.add = 0;
        self.vals.push(val as i64);
    }
    fn add_all(&mut self, inc: i32) { self.add = (self.add + inc as i64) % MOD; }
    fn mult_all(&mut self, m: i32) {
        let m = m as i64;
        self.mul = self.mul * m % MOD;
        self.add = self.add * m % MOD;
    }
    fn get_index(&self, idx: i32) -> i32 {
        if idx as usize >= self.vals.len() { return -1; }
        ((self.vals[idx as usize] * self.mul % MOD + self.add) % MOD) as i32
    }
}
fn main() {
    let mut f = Fancy::new();
    f.append(2);
    println!("{}", f.get_index(0));
}
#[cfg(test)]
mod tests {
    use super::Fancy;
    #[test]
    fn example_one() {
        let mut f = Fancy::new();
        f.append(2);
        f.add_all(3);
        f.mult_all(2);
        assert_eq!(f.get_index(0), 10);
        f.append(3);
        assert_eq!(f.get_index(1), 3);
        assert_eq!(f.get_index(0), 10);
    }
}