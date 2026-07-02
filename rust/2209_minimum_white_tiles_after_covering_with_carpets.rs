/// LeetCode #2209 - Minimum White Tiles After Covering With Carpets
fn minimum_white_tiles(floor: String, num_carpets: i32, carpet_len: i32) -> i32 {
    let floor = floor.as_bytes();
    let n = floor.len();
    let num_carpets = num_carpets as usize;
    let carpet_len = carpet_len as usize;
    let mut dp = vec![vec![0i32; num_carpets + 1]; n + 1];

    for i in (0..n).rev() {
        dp[i][0] = (floor[i] == b'1') as i32 + dp[i + 1][0];
    }

    for i in (0..n).rev() {
        for j in 1..=num_carpets {
            let cover = if i + carpet_len < n {
                dp[i + carpet_len][j - 1]
            } else {
                0
            };
            let skip = (floor[i] == b'1') as i32 + dp[i + 1][j];
            dp[i][j] = cover.min(skip);
        }
    }

    dp[0][num_carpets]
}

fn main() {
    println!("{}", minimum_white_tiles("10110101".into(), 2, 2));
}

#[cfg(test)]
mod tests {
    use super::minimum_white_tiles;

    #[test]
    fn example_one() {
        assert_eq!(minimum_white_tiles("10110101".into(), 2, 2), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_white_tiles("11111".into(), 2, 3), 0);
    }
}
