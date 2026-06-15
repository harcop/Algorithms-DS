/// LeetCode #1861 - Rotating the Box
use std::collections::VecDeque;

fn rotate_the_box(box_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let m = box_grid.len();
    let n = box_grid[0].len();
    let mut ans = vec![vec!['.'; m]; n];
    for i in 0..m {
        for j in 0..n {
            ans[j][m - 1 - i] = box_grid[i][j];
        }
    }
    for j in 0..m {
        let mut q = VecDeque::new();
        for i in (0..n).rev() {
            match ans[i][j] {
                '*' => q.clear(),
                '.' => q.push_back(i),
                '#' => {
                    if let Some(pos) = q.pop_front() {
                        ans[pos][j] = '#';
                        ans[i][j] = '.';
                        q.push_back(i);
                    }
                }
                _ => {}
            }
        }
    }
    ans
}

fn main() {
    let grid = vec![vec!['#', '.', '#']];
    println!("{:?}", rotate_the_box(grid));
}

#[cfg(test)]
mod tests {
    use super::rotate_the_box;

    #[test]
    fn example_one() {
        assert_eq!(
            rotate_the_box(vec![vec!['#', '.', '#']]),
            vec![vec!['.'], vec!['#'], vec!['#']]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            rotate_the_box(vec![
                vec!['#', '.', '*', '.'],
                vec!['#', '#', '*', '.']
            ]),
            vec![
                vec!['#', '.'],
                vec!['#', '#'],
                vec!['*', '*'],
                vec!['.', '.']
            ]
        );
    }
}
