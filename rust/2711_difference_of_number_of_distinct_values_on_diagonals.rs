/// LeetCode #2711 - Difference of Number of Distinct Values on Diagonals
use std::collections::HashSet;

fn difference_of_distinct_values(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    let mut ans = vec![vec![0; n]; m];
    for i in 0..m {
        for j in 0..n {
            let mut s = HashSet::new();
            let mut x = i;
            let mut y = j;
            while x > 0 && y > 0 {
                x -= 1;
                y -= 1;
                s.insert(grid[x][y]);
            }
            let tl = s.len() as i32;
            s.clear();
            x = i;
            y = j;
            while x + 1 < m && y + 1 < n {
                x += 1;
                y += 1;
                s.insert(grid[x][y]);
            }
            let br = s.len() as i32;
            ans[i][j] = (tl - br).abs();
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        difference_of_distinct_values(vec![vec![1, 2, 3], vec![3, 1, 5], vec![3, 2, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::difference_of_distinct_values;

    #[test]
    fn example_one() {
        assert_eq!(
            difference_of_distinct_values(vec![vec![1, 2, 3], vec![3, 1, 5], vec![3, 2, 1]]),
            vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 1, 1]]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(difference_of_distinct_values(vec![vec![1]]), vec![vec![0]]);
    }
}
