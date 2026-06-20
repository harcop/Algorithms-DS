/// LeetCode #2001 - Number of Pairs of Interchangeable Rectangles
use std::collections::HashMap;

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
    let mut cnt: HashMap<(i32, i32), i64> = HashMap::new();
    let mut ans = 0i64;
    for r in rectangles {
        let (mut w, mut h) = (r[0], r[1]);
        let g = gcd(w, h);
        w /= g;
        h /= g;
        ans += cnt.get(&(w, h)).copied().unwrap_or(0);
        *cnt.entry((w, h)).or_insert(0) += 1;
    }
    ans
}

fn main() {
    println!(
        "{}",
        interchangeable_rectangles(vec![vec![4, 8], vec![3, 6], vec![10, 20], vec![15, 30]])
    );
}

#[cfg(test)]
mod tests {
    use super::interchangeable_rectangles;

    #[test]
    fn example_one() {
        assert_eq!(
            interchangeable_rectangles(vec![vec![4, 8], vec![3, 6], vec![10, 20], vec![15, 30]]),
            6
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            interchangeable_rectangles(vec![vec![4, 5], vec![7, 8]]),
            0
        );
    }
}
