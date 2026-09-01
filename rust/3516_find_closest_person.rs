/// LeetCode #3516 - Find Closest Person
fn find_closest(x: i32, y: i32, z: i32) -> i32 {
    let a = (x - z).abs();
    let b = (y - z).abs();
    if a == b {
        0
    } else if a < b {
        1
    } else {
        2
    }
}

fn main() {
    println!("{}", find_closest(2, 7, 4));
}

#[cfg(test)]
mod tests {
    use super::find_closest;

    #[test]
    fn example1() {
        assert_eq!(find_closest(2, 7, 4), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(find_closest(2, 5, 6), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(find_closest(1, 5, 3), 0);
    }
}
