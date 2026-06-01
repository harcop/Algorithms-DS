/// LeetCode #1641 - Count Sorted Vulnerable Squares In Chessboard
fn letter(r: i32, c: i32) -> u8 { (b'a' + ((r + c - 2) % 26) as u8) }

fn is_sorted(r: i32, c: i32) -> bool {
    let ch = letter(r, c);
    for k in 1..c {
        if letter(r, k) > ch { return false; }
    }
    for k in 1..r {
        if letter(k, c) > ch { return false; }
    }
    true
}

fn count_sorted_squares(coordinates: Vec<Vec<i32>>) -> i32 {
    coordinates.iter().filter(|p| is_sorted(p[0], p[1])).count() as i32
}
fn main() { println!("{}", count_sorted_squares(vec![vec![1,1],vec![8,8]])); }
#[cfg(test)]
mod tests {
    use super::count_sorted_squares;
    #[test]
    fn example_one() { assert_eq!(count_sorted_squares(vec![vec![1,1],vec![8,8],vec![5,6]]), 3); }
}