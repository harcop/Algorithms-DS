/// LeetCode #3424 - Minimum Cost to Make Arrays Identical
fn min_cost(arr: Vec<i32>, brr: Vec<i32>, k: i64) -> i64 {
    let c1: i64 = arr
        .iter()
        .zip(brr.iter())
        .map(|(&a, &b)| (a as i64 - b as i64).abs())
        .sum();
    let mut a = arr;
    let mut b = brr;
    a.sort_unstable();
    b.sort_unstable();
    let c2 = k
        + a.iter()
            .zip(b.iter())
            .map(|(&x, &y)| (x as i64 - y as i64).abs())
            .sum::<i64>();
    c1.min(c2)
}

fn main() {
    println!("{}", min_cost(vec![-7, 9, 5], vec![7, -2, -5], 2));
}

#[cfg(test)]
mod tests {
    use super::min_cost;

    #[test]
    fn example1() {
        assert_eq!(min_cost(vec![-7, 9, 5], vec![7, -2, -5], 2), 13);
    }

    #[test]
    fn example2() {
        assert_eq!(min_cost(vec![2, 1], vec![2, 1], 0), 0);
    }
}
