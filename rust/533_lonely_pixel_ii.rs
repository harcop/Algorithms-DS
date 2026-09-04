/// LeetCode #533 - Lonely Pixel II
use std::collections::HashMap;

fn find_lonely_pixel(picture: Vec<Vec<char>>, n: i32) -> i32 {
    let m = picture.len();
    let cols = picture[0].len();
    let n = n as usize;
    let mut row_str = vec![String::new(); m];
    let mut row_count = vec![0usize; m];
    let mut col_count = vec![0usize; cols];
    let mut freq: HashMap<String, usize> = HashMap::new();
    for i in 0..m {
        let s: String = picture[i].iter().collect();
        let blacks = picture[i].iter().filter(|&&c| c == 'B').count();
        row_str[i] = s.clone();
        row_count[i] = blacks;
        *freq.entry(s).or_insert(0) += 1;
        for j in 0..cols {
            if picture[i][j] == 'B' {
                col_count[j] += 1;
            }
        }
    }
    let mut ans = 0;
    for i in 0..m {
        if row_count[i] != n {
            continue;
        }
        if *freq.get(&row_str[i]).unwrap_or(&0) != n {
            continue;
        }
        for j in 0..cols {
            if picture[i][j] == 'B' && col_count[j] == n {
                ans += 1;
            }
        }
    }
    ans as i32
}

fn main() {
    let picture = vec![
        vec!['W', 'B', 'W', 'B', 'B', 'W'],
        vec!['W', 'B', 'W', 'B', 'B', 'W'],
        vec!['W', 'B', 'W', 'B', 'B', 'W'],
        vec!['W', 'W', 'B', 'W', 'B', 'W'],
    ];
    println!("{}", find_lonely_pixel(picture, 3));
}

#[cfg(test)]
mod tests {
    use super::find_lonely_pixel;

    #[test]
    fn example() {
        let picture = vec![
            vec!['W', 'B', 'W', 'B', 'B', 'W'],
            vec!['W', 'B', 'W', 'B', 'B', 'W'],
            vec!['W', 'B', 'W', 'B', 'B', 'W'],
            vec!['W', 'W', 'B', 'W', 'B', 'W'],
        ];
        assert_eq!(find_lonely_pixel(picture, 3), 6);
    }
}
