/// LeetCode #970 - Powerful Integers
use std::collections::HashSet;

fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    let mut xi = 1i64;
    while xi <= bound as i64 {
        let mut yj = 1i64;
        while xi + yj <= bound as i64 {
            let s = (xi + yj) as i32;
            if seen.insert(s) {
                out.push(s);
            }
            if y == 1 {
                break;
            }
            yj *= y as i64;
        }
        if x == 1 {
            break;
        }
        xi *= x as i64;
    }
    out.sort_unstable();
    out
}

fn main() {
    println!("{:?}", powerful_integers(2, 3, 10));
}

#[cfg(test)]
mod tests {
    use super::powerful_integers;

    #[test]
    fn example_one() {
        assert_eq!(powerful_integers(2, 3, 10), vec![2, 3, 4, 5, 7, 9, 10]);
    }

    #[test]
    fn example_two() {
        assert_eq!(powerful_integers(3, 5, 15), vec![2, 4, 6, 8, 10, 14]);
    }
}
