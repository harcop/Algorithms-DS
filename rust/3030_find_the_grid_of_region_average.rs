/// LeetCode #3030 - Find the Grid of Region Average
fn valid_region(image: &[Vec<i32>], r: usize, c: usize, threshold: i32) -> bool {
    for i in r..r + 3 {
        for j in c..c + 3 {
            if j + 1 < c + 3 && (image[i][j] - image[i][j + 1]).abs() > threshold {
                return false;
            }
            if i + 1 < r + 3 && (image[i][j] - image[i + 1][j]).abs() > threshold {
                return false;
            }
        }
    }
    true
}

fn result_grid(image: Vec<Vec<i32>>, threshold: i32) -> Vec<Vec<i32>> {
    let rows = image.len();
    let cols = image[0].len();
    let mut sum = vec![vec![0i64; cols]; rows];
    let mut cnt = vec![vec![0i32; cols]; rows];

    for r in 0..=rows.saturating_sub(3) {
        for c in 0..=cols.saturating_sub(3) {
            if !valid_region(&image, r, c, threshold) {
                continue;
            }
            let mut tot = 0i64;
            for i in r..r + 3 {
                for j in c..c + 3 {
                    tot += image[i][j] as i64;
                }
            }
            let avg = tot / 9;
            for i in r..r + 3 {
                for j in c..c + 3 {
                    sum[i][j] += avg;
                    cnt[i][j] += 1;
                }
            }
        }
    }

    let mut result = image;
    for i in 0..rows {
        for j in 0..cols {
            if cnt[i][j] > 0 {
                result[i][j] = (sum[i][j] / cnt[i][j] as i64) as i32;
            }
        }
    }
    result
}

fn main() {
    let image = vec![
        vec![5, 6, 7, 10],
        vec![8, 9, 10, 10],
        vec![11, 12, 13, 10],
    ];
    println!("{:?}", result_grid(image, 3));
}

#[cfg(test)]
mod tests {
    use super::result_grid;

    #[test]
    fn example1() {
        let image = vec![
            vec![5, 6, 7, 10],
            vec![8, 9, 10, 10],
            vec![11, 12, 13, 10],
        ];
        let expected = vec![
            vec![9, 9, 9, 9],
            vec![9, 9, 9, 9],
            vec![9, 9, 9, 9],
        ];
        assert_eq!(result_grid(image, 3), expected);
    }
}
