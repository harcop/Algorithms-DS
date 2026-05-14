/// LeetCode #764 - Largest Plus Sign
use std::collections::HashSet;

fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut ban: HashSet<(usize, usize)> = HashSet::new();
    for m in mines {
        ban.insert((m[0] as usize, m[1] as usize));
    }
    let mut left = vec![vec![0i32; n]; n];
    let mut right = vec![vec![0i32; n]; n];
    let mut up = vec![vec![0i32; n]; n];
    let mut down = vec![vec![0i32; n]; n];
    for i in 0..n {
        for j in 0..n {
            if ban.contains(&(i, j)) {
                left[i][j] = 0;
            } else {
                left[i][j] = if j == 0 {
                    1
                } else {
                    left[i][j - 1] + 1
                };
            }
        }
        for j in (0..n).rev() {
            if ban.contains(&(i, j)) {
                right[i][j] = 0;
            } else {
                right[i][j] = if j + 1 == n {
                    1
                } else {
                    right[i][j + 1] + 1
                };
            }
        }
    }
    for j in 0..n {
        for i in 0..n {
            if ban.contains(&(i, j)) {
                up[i][j] = 0;
            } else {
                up[i][j] = if i == 0 {
                    1
                } else {
                    up[i - 1][j] + 1
                };
            }
        }
        for i in (0..n).rev() {
            if ban.contains(&(i, j)) {
                down[i][j] = 0;
            } else {
                down[i][j] = if i + 1 == n {
                    1
                } else {
                    down[i + 1][j] + 1
                };
            }
        }
    }
    let mut ans = 0i32;
    for i in 0..n {
        for j in 0..n {
            if !ban.contains(&(i, j)) {
                let v = left[i][j]
                    .min(right[i][j])
                    .min(up[i][j])
                    .min(down[i][j]);
                ans = ans.max(v);
            }
        }
    }
    ans
}

fn main() {
    println!("{}", order_of_largest_plus_sign(5, vec![vec![4, 2]]));
}

#[cfg(test)]
mod tests {
    use super::order_of_largest_plus_sign;

    #[test]
    fn example_one() {
        assert_eq!(order_of_largest_plus_sign(5, vec![vec![4, 2]]), 2);
    }
}
