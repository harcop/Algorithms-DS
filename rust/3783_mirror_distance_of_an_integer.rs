/// LeetCode #3783 - Mirror Distance of an Integer
fn mirror_distance(n: i32) -> i32 {
    let mut x = n;
    let mut y = 0;
    while x > 0 {
        y = y * 10 + x % 10;
        x /= 10;
    }
    (n - y).abs()
}

fn main() {
    println!("{}", mirror_distance(25));
}

#[cfg(test)]
mod tests {
    use super::mirror_distance;

    #[test]
    fn example1() {
        assert_eq!(mirror_distance(25), 27);
    }

    #[test]
    fn example2() {
        assert_eq!(mirror_distance(10), 9);
    }

    #[test]
    fn example3() {
        assert_eq!(mirror_distance(7), 0);
    }
}
