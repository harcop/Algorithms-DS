/// LeetCode #3492 - Maximum Containers on a Ship
fn max_containers(n: i32, w: i32, max_weight: i32) -> i32 {
    let n = n as i64;
    let w = w as i64;
    let max_weight = max_weight as i64;
    (n * n * w).min(max_weight) as i32 / w as i32
}

fn main() {
    println!("{}", max_containers(2, 3, 15));
}

#[cfg(test)]
mod tests {
    use super::max_containers;

    #[test]
    fn example1() {
        assert_eq!(max_containers(2, 3, 15), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(max_containers(3, 5, 20), 4);
    }
}
