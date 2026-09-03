/// LeetCode #3560 - Find Minimum Log Transportation Cost
fn min_cutting_cost(n: i32, m: i32, k: i32) -> i64 {
    let x = n.max(m);
    if x <= k {
        0
    } else {
        k as i64 * (x - k) as i64
    }
}

fn main() {
    println!("{}", min_cutting_cost(6, 5, 5));
}

#[cfg(test)]
mod tests {
    use super::min_cutting_cost;

    #[test]
    fn example1() {
        assert_eq!(min_cutting_cost(6, 5, 5), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(min_cutting_cost(4, 4, 6), 0);
    }
}
