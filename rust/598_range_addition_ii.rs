/// LeetCode #598 - Range Addition II
fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
    if ops.is_empty() {
        return m * n;
    }
    let mut a = i32::MAX;
    let mut b = i32::MAX;
    for op in ops {
        a = a.min(op[0]);
        b = b.min(op[1]);
    }
    a * b
}

fn main() {
    println!("{}", max_count(3, 3, vec![vec![2, 2], vec![3, 3]]));
}

#[cfg(test)]
mod tests {
    use super::max_count;

    #[test]
    fn example_one() {
        assert_eq!(max_count(3, 3, vec![vec![2, 2], vec![3, 3]]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            max_count(3, 3, vec![vec![2, 2], vec![3, 3], vec![3, 3], vec![3, 3], vec![2, 2], vec![3, 3], vec![3, 3], vec![3, 3], vec![2, 2], vec![3, 3], vec![3, 3], vec![3, 3]]),
            4
        );
    }
}
