/// LeetCode #4061 - Minimum Queen Moves to Reach Target
fn min_queen_moves(source: Vec<i32>, target: Vec<i32>) -> i32 {
    let (sr, sc) = (source[0], source[1]);
    let (tr, tc) = (target[0], target[1]);
    if sr == tr && sc == tc {
        return 0;
    }
    if sr == tr || sc == tc || (sr - tr).abs() == (sc - tc).abs() {
        return 1;
    }
    2
}

fn main() {
    println!("{}", min_queen_moves(vec![8, 1], vec![1, 8]));
}

#[cfg(test)]
mod tests {
    use super::min_queen_moves;

    #[test]
    fn example1() {
        assert_eq!(min_queen_moves(vec![8, 1], vec![1, 8]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(min_queen_moves(vec![4, 2], vec![1, 3]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(min_queen_moves(vec![1, 1], vec![1, 1]), 0);
    }
}
