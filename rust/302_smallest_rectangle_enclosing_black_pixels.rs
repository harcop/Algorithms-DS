/// LeetCode #302 - Smallest Rectangle Enclosing Black Pixels
fn min_area(image: Vec<Vec<char>>, x: i32, y: i32) -> i32 {
    let m = image.len();
    let n = image[0].len();
    let mut top = x as usize;
    let mut bot = x as usize;
    let mut left = y as usize;
    let mut right = y as usize;
    for i in 0..m {
        for j in 0..n {
            if image[i][j] == '1' {
                top = top.min(i);
                bot = bot.max(i);
                left = left.min(j);
                right = right.max(j);
            }
        }
    }
    ((bot - top + 1) * (right - left + 1)) as i32
}

fn main() {
    let img = vec![
        vec!['0', '0', '1', '0'],
        vec!['0', '1', '1', '0'],
        vec!['0', '1', '1', '0'],
    ];
    println!("{}", min_area(img, 0, 2));
}

#[cfg(test)]
mod tests {
    use super::min_area;

    #[test]
    fn example_one() {
        let img = vec![
            vec!['0', '0', '1', '0'],
            vec!['0', '1', '1', '0'],
            vec!['0', '1', '1', '0'],
        ];
        assert_eq!(min_area(img, 0, 2), 6);
    }
}
