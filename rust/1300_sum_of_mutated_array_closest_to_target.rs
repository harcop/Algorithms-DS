/// LeetCode #1300 - Sum of Mutated Array Closest to Target
fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {
    let maxv = *arr.iter().max().unwrap_or(&0);
    let mut best = 0;
    let mut best_diff = i64::MAX;
    for v in 0..=maxv {
        let s: i64 = arr.iter().map(|&x| (x.min(v)) as i64).sum();
        let d = (s - target as i64).abs();
        if d < best_diff || (d == best_diff && v < best) {
            best_diff = d;
            best = v;
        }
    }
    best
}

fn main() {
    println!("{}", find_best_value(vec![4, 9, 3], 10));
}

#[cfg(test)]
mod tests {
    use super::find_best_value;

    #[test]
    fn example_one() {
        assert_eq!(find_best_value(vec![4, 9, 3], 10), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_best_value(vec![2, 3, 5], 10), 5);
    }
}
