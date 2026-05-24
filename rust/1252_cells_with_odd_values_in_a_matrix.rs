/// LeetCode #1252 - Cells with Odd Values in a Matrix
fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let mut row = vec![0i32; m];
    let mut col = vec![0i32; n];
    for idx in indices {
        row[idx[0] as usize] ^= 1;
        col[idx[1] as usize] ^= 1;
    }
    let mut ans = 0i32;
    for r in 0..m {
        for c in 0..n {
            if (row[r] ^ col[c]) == 1 {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", odd_cells(2, 3, vec![vec![0, 1], vec![1, 2]]));
}

#[cfg(test)]
mod tests {
    use super::odd_cells;

    #[test]
    fn example_one() {
        assert_eq!(odd_cells(2, 3, vec![vec![0, 1], vec![1, 2]]), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(odd_cells(2, 2, vec![vec![1, 1], vec![0, 0]]), 0);
    }
}
