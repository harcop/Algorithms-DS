/// LeetCode #661 - Image Smoother
fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = img.len();
    let n = img[0].len();
    let mut out = vec![vec![0i32; n]; m];
    for i in 0..m {
        for j in 0..n {
            let mut sum = 0i32;
            let mut cnt = 0i32;
            for di in -1i32..=1 {
                for dj in -1i32..=1 {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 {
                        sum += img[ni as usize][nj as usize];
                        cnt += 1;
                    }
                }
            }
            out[i][j] = sum / cnt;
        }
    }
    out
}

fn main() {
    println!("{:?}", image_smoother(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]));
}

#[cfg(test)]
mod tests {
    use super::image_smoother;

    #[test]
    fn example_one() {
        assert_eq!(
            image_smoother(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
            vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]
        );
    }
}
