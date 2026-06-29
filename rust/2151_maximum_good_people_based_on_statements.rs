/// LeetCode #2151 - Maximum Good People Based on Statements
fn maximum_good(statements: Vec<Vec<i32>>) -> i32 {
    let n = statements.len();
    let mut ans = 0i32;

    'mask: for mask in 0..(1usize << n) {
        for i in 0..n {
            if (mask >> i) & 1 == 0 {
                continue;
            }
            for j in 0..n {
                let statement = statements[i][j];
                if statement != 2 && statement as usize != ((mask >> j) & 1) {
                    continue 'mask;
                }
            }
        }
        ans = ans.max(mask.count_ones() as i32);
    }

    ans
}

fn main() {
    println!(
        "{}",
        maximum_good(vec![vec![2, 1, 2], vec![1, 2, 2], vec![2, 0, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_good;

    #[test]
    fn example_one() {
        assert_eq!(
            maximum_good(vec![vec![2, 1, 2], vec![1, 2, 2], vec![2, 0, 2]]),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_good(vec![vec![2, 0], vec![0, 2]]), 1);
    }
}
