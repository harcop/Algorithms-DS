/// LeetCode #1349 - Maximum Students Taking Exam

fn max_students(seats: Vec<Vec<char>>) -> i32 {
    let m = seats.len();
    let n = seats[0].len();
    let mut row_masks = vec![0i32; m];
    for i in 0..m {
        for j in 0..n {
            if seats[i][j] == '.' {
                row_masks[i] |= 1 << j;
            }
        }
    }
    let mut dp = vec![0i32; 1 << n];
    for mask in 0usize..(1 << n) {
        if mask & !row_masks[0] as usize == 0 && valid(mask as i32, n) {
            dp[mask] = mask.count_ones() as i32;
        }
    }
    for i in 1..m {
        let mut ndp = vec![0i32; 1 << n];
        for cur in 0usize..(1 << n) {
            if cur & !row_masks[i] as usize != 0 || !valid(cur as i32, n) {
                continue;
            }
            for prev in 0usize..(1 << n) {
                if prev & cur != 0 {
                    continue;
                }
                if conflict(prev as i32, cur as i32, n) {
                    continue;
                }
                ndp[cur] = ndp[cur].max(dp[prev] + cur.count_ones() as i32);
            }
        }
        dp = ndp;
    }
    *dp.iter().max().unwrap_or(&0)
}

fn valid(mask: i32, n: usize) -> bool {
    for j in 0..n {
        if mask & (1 << j) != 0 && j > 0 && mask & (1 << (j - 1)) != 0 {
            return false;
        }
    }
    true
}

fn conflict(prev: i32, cur: i32, n: usize) -> bool {
    for j in 0..n {
        if cur & (1 << j) != 0 {
            if j > 0 && prev & (1 << (j - 1)) != 0 {
                return true;
            }
            if j + 1 < n && prev & (1 << (j + 1)) != 0 {
                return true;
            }
        }
    }
    false
}

fn main() {
    let seats = vec![
        vec!['#', '.', '#', '#', '.', '#'],
        vec!['.', '#', '#', '#', '#', '.'],
        vec!['#', '.', '#', '#', '.', '#'],
    ];
    println!("{}", max_students(seats));
}

#[cfg(test)]
mod tests {
    use super::max_students;

    #[test]
    fn example_one() {
        let seats = vec![
            vec!['#', '.', '#', '#', '.', '#'],
            vec!['.', '#', '#', '#', '#', '.'],
            vec!['#', '.', '#', '#', '.', '#'],
        ];
        assert_eq!(max_students(seats), 4);
    }
}
