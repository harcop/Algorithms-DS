/// LeetCode #531 - Lonely Pixel I
fn find_lonely_pixel(picture: Vec<Vec<char>>) -> i32 {
    let m = picture.len();
    let n = picture[0].len();
    let mut row = vec![0; m];
    let mut col = vec![0; n];
    for i in 0..m {
        for j in 0..n {
            if picture[i][j] == 'B' {
                row[i] += 1;
                col[j] += 1;
            }
        }
    }
    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            if picture[i][j] == 'B' && row[i] == 1 && col[j] == 1 {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    let picture = vec![
        vec!['W', 'W', 'B'],
        vec!['W', 'B', 'W'],
        vec!['B', 'W', 'W'],
    ];
    println!("{}", find_lonely_pixel(picture));
}

#[cfg(test)]
mod tests {
    use super::find_lonely_pixel;

    #[test]
    fn example() {
        let picture = vec![
            vec!['W', 'W', 'B'],
            vec!['W', 'B', 'W'],
            vec!['B', 'W', 'W'],
        ];
        assert_eq!(find_lonely_pixel(picture), 3);
    }
}
