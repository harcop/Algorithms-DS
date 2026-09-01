/// LeetCode #3546 - Equal Sum Grid Partition I
fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
    let total: i64 = grid.iter().flat_map(|row| row.iter().map(|&x| x as i64)).sum();
    if total % 2 != 0 {
        return false;
    }
    let mut pre = 0i64;
    for (i, row) in grid.iter().enumerate() {
        pre += row.iter().map(|&x| x as i64).sum::<i64>();
        if pre * 2 == total && i + 1 != grid.len() {
            return true;
        }
    }
    let n = grid[0].len();
    pre = 0;
    for j in 0..n {
        pre += grid.iter().map(|row| row[j] as i64).sum::<i64>();
        if pre * 2 == total && j + 1 != n {
            return true;
        }
    }
    false
}

fn main() {
    println!("{}", can_partition_grid(vec![vec![1, 4], vec![2, 3]]));
}

#[cfg(test)]
mod tests {
    use super::can_partition_grid;

    #[test]
    fn example1() {
        assert_eq!(can_partition_grid(vec![vec![1, 4], vec![2, 3]]), true);
    }

    #[test]
    fn example2() {
        assert_eq!(can_partition_grid(vec![vec![1, 3], vec![2, 4]]), false);
    }
}
