/// LeetCode #1033 - Moving Stones Until Consecutive
fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
    let mut v = vec![a, b, c];
    v.sort_unstable();
    let (x, y, z) = (v[0], v[1], v[2]);
    let mut min_moves = 0i32;
    let mut max_moves = z - x - 2;
    if z - y == 1 && y - x == 1 {
        min_moves = 0;
    } else if z - y <= 2 || y - x <= 2 {
        min_moves = 1;
    } else {
        min_moves = 2;
    }
    if y - x == 1 {
        max_moves = z - y - 1;
    }
    if z - y == 1 {
        max_moves = y - x - 1;
    }
    vec![min_moves, max_moves]
}

fn main() {
    println!("{:?}", num_moves_stones(1, 2, 5));
}

#[cfg(test)]
mod tests {
    use super::num_moves_stones;

    #[test]
    fn example_one() {
        assert_eq!(num_moves_stones(1, 2, 5), vec![1, 3]);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_moves_stones(4, 3, 2), vec![0, 0]);
    }
}
