/// LeetCode #2923 - Find Champion I
fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    for (i, row) in grid.iter().enumerate() {
        if (0..n).filter(|&j| j != i).all(|j| row[j] == 1) {
            return i as i32;
        }
    }
    -1
}

fn main() {
    println!("{}", find_champion(vec![vec![0, 1], vec![0, 0]]));
}

#[cfg(test)]
mod tests {
    use super::find_champion;

    #[test]
    fn example_one() {
        assert_eq!(find_champion(vec![vec![0, 1], vec![0, 0]]), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_champion(vec![vec![0, 0, 1], vec![1, 0, 1], vec![0, 0, 0]]),
            1
        );
    }
}
