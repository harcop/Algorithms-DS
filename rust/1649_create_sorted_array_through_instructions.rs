/// LeetCode #1649 - Create Sorted Array Through Instructions
struct Fenwick {
    n: usize,
    bit: Vec<i32>,
}
impl Fenwick {
    fn new(n: usize) -> Self { Self { n, bit: vec![0; n + 1] } }
    fn add(&mut self, mut i: usize, v: i32) {
        i += 1;
        while i <= self.n {
            self.bit[i] += v;
            i += i & i.wrapping_neg();
        }
    }
    fn sum(&self, mut i: usize) -> i32 {
        let mut s = 0i32;
        i += 1;
        while i > 0 {
            s += self.bit[i];
            i -= i & i.wrapping_neg();
        }
        s
    }
}

fn create_sorted_array(instructions: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut fw = Fenwick::new(100_001);
    let mut ans = 0i64;
    for x in instructions {
        let x = x as usize;
        let less = fw.sum(x.saturating_sub(1)) as i64;
        let total = fw.sum(100_000) as i64;
        let greater = total - fw.sum(x) as i64;
        ans = (ans + less.min(greater)) % MOD;
        fw.add(x, 1);
    }
    ans as i32
}
fn main() { println!("{}", create_sorted_array(vec![1,5,6,2])); }
#[cfg(test)]
mod tests {
    use super::create_sorted_array;
    #[test]
    fn example_one() { assert_eq!(create_sorted_array(vec![1,5,6,2]), 1); }
}