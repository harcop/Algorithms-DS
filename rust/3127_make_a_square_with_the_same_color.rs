/// LeetCode #3127 - Make a Square with the Same Color
fn can_make_square(grid: Vec<Vec<char>>) -> bool {
    let dirs = [0, 0, 1, 1, 0];
    for i in 0..2 {
        for j in 0..2 {
            let mut cnt1 = 0;
            let mut cnt2 = 0;
            for k in 0..4 {
                let x = i + dirs[k];
                let y = j + dirs[k + 1];
                if grid[x][y] == 'W' {
                    cnt1 += 1;
                } else {
                    cnt2 += 1;
                }
            }
            if cnt1 != cnt2 {
                return true;
            }
        }
    }
    false
}

fn main() {
    println!(
        "{}",
        can_make_square(vec![
            vec!['B', 'W', 'B'],
            vec!['B', 'W', 'W'],
            vec!['B', 'W', 'B']
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::can_make_square;

    #[test]
    fn example1() {
        assert!(can_make_square(vec![
            vec!['B', 'W', 'B'],
            vec!['B', 'W', 'W'],
            vec!['B', 'W', 'B']
        ]));
    }

    #[test]
    fn example2() {
        assert!(!can_make_square(vec![
            vec!['B', 'W', 'B'],
            vec!['W', 'B', 'W'],
            vec!['B', 'W', 'B']
        ]));
    }

    #[test]
    fn example3() {
        assert!(can_make_square(vec![
            vec!['B', 'W', 'B'],
            vec!['B', 'W', 'W'],
            vec!['B', 'W', 'W']
        ]));
    }
}
