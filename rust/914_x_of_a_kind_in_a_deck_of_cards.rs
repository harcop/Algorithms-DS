/// LeetCode #914 - X of a Kind in a Deck of Cards
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.abs()
}

fn has_groups_size_x(deck: Vec<i32>) -> bool {
    use std::collections::HashMap;
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for x in deck {
        *cnt.entry(x).or_insert(0) += 1;
    }
    let mut g = 0;
    for &c in cnt.values() {
        g = if g == 0 { c } else { gcd(g, c) };
    }
    g >= 2
}

fn main() {
    println!("{}", has_groups_size_x(vec![1, 1, 2, 2, 2, 2]));
}

#[cfg(test)]
mod tests {
    use super::has_groups_size_x;

    #[test]
    fn example_one() {
        assert!(has_groups_size_x(vec![1, 1, 2, 2, 2, 2]));
    }

    #[test]
    fn example_two() {
        assert!(!has_groups_size_x(vec![1, 1, 1, 2, 2, 2, 3, 3]));
    }

    #[test]
    fn singleton_makes_impossible() {
        assert!(!has_groups_size_x(vec![1, 2, 3, 4, 4, 3, 2, 4]));
    }
}
