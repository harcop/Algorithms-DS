/// LeetCode #3259 - Maximum Energy Boost From Two Drinks
fn max_energy_boost(energy_drink_a: Vec<i32>, energy_drink_b: Vec<i32>) -> i64 {
    let n = energy_drink_a.len();
    let mut a = energy_drink_a[0] as i64;
    let mut b = energy_drink_b[0] as i64;
    for i in 1..n {
        let na = (a + energy_drink_a[i] as i64).max(b);
        let nb = (b + energy_drink_b[i] as i64).max(a);
        a = na;
        b = nb;
    }
    a.max(b)
}

fn main() {
    println!("{}", max_energy_boost(vec![1, 3, 1], vec![3, 1, 1]));
}

#[cfg(test)]
mod tests {
    use super::max_energy_boost;

    #[test]
    fn example1() {
        assert_eq!(max_energy_boost(vec![1, 3, 1], vec![3, 1, 1]), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(max_energy_boost(vec![4, 1, 1], vec![1, 1, 3]), 7);
    }
}
