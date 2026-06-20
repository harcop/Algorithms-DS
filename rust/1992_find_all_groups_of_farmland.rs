/// LeetCode #1992 - Find All Groups of Farmland
fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = land.len();
    let n = land[0].len();
    let mut ans = Vec::new();
    for i in 0..m {
        for j in 0..n {
            if land[i][j] == 0
                || (j > 0 && land[i][j - 1] == 1)
                || (i > 0 && land[i - 1][j] == 1)
            {
                continue;
            }
            let mut x = i;
            let mut y = j;
            while x + 1 < m && land[x + 1][j] == 1 {
                x += 1;
            }
            while y + 1 < n && land[x][y + 1] == 1 {
                y += 1;
            }
            ans.push(vec![i as i32, j as i32, x as i32, y as i32]);
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        find_farmland(vec![vec![1, 0, 0], vec![0, 1, 1], vec![0, 1, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::find_farmland;

    #[test]
    fn example_one() {
        assert_eq!(
            find_farmland(vec![vec![1, 0, 0], vec![0, 1, 1], vec![0, 1, 1]]),
            vec![vec![0, 0, 0, 0], vec![1, 1, 2, 2]]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_farmland(vec![vec![1, 1], vec![1, 1]]),
            vec![vec![0, 0, 1, 1]]
        );
    }
}
