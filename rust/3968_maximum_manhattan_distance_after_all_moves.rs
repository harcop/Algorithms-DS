/// LeetCode #3968 - Maximum Manhattan Distance After All Moves
fn max_distance(moves: String) -> i32 {
    let mut x = 0i32;
    let mut y = 0i32;
    let mut wild = 0i32;
    for c in moves.bytes() {
        match c {
            b'U' => x -= 1,
            b'D' => x += 1,
            b'L' => y -= 1,
            b'R' => y += 1,
            _ => wild += 1,
        }
    }
    x.abs() + y.abs() + wild
}

fn main() {
    println!("{}", max_distance("L_D_".into()));
}

#[cfg(test)]
mod tests {
    use super::max_distance;

    #[test]
    fn example1() {
        assert_eq!(max_distance("L_D_".into()), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(max_distance("U_R".into()), 3);
    }
}
