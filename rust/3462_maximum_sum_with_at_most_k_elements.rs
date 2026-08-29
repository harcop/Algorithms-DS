/// LeetCode #3462 - Maximum Sum With at Most K Elements
fn max_sum(mut grid: Vec<Vec<i32>>, limits: Vec<i32>, k: i32) -> i64 {
    let mut cand = Vec::new();
    for (row, &lim) in grid.iter_mut().zip(limits.iter()) {
        row.sort_unstable();
        for _ in 0..lim {
            if let Some(x) = row.pop() {
                cand.push(x);
            }
        }
    }
    cand.sort_unstable();
    cand.iter()
        .rev()
        .take(k as usize)
        .map(|&x| x as i64)
        .sum()
}

fn main() {
    println!("{}", max_sum(vec![vec![1, 2], vec![3, 4]], vec![1, 2], 2));
}

#[cfg(test)]
mod tests {
    use super::max_sum;

    #[test]
    fn example1() {
        assert_eq!(max_sum(vec![vec![1, 2], vec![3, 4]], vec![1, 2], 2), 7);
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_sum(vec![vec![5, 3, 7], vec![8, 2, 6]], vec![2, 2], 3),
            21
        );
    }
}
