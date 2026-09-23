/// LeetCode #3890 - Integers With Multiple Sum of Two Cubes
use std::collections::HashMap;

fn two_cube_sums(n: i64) -> Vec<i64> {
    let mut cnt: HashMap<i64, i32> = HashMap::new();
    for a in 1..=1000i64 {
        let a3 = a * a * a;
        for b in a..=1000 {
            let sum = a3 + b * b * b;
            *cnt.entry(sum).or_insert(0) += 1;
        }
    }
    let mut out: Vec<i64> = cnt
        .into_iter()
        .filter(|(_, c)| *c >= 2)
        .map(|(k, _)| k)
        .filter(|&k| k <= n)
        .collect();
    out.sort_unstable();
    out
}

fn main() {
    println!("{:?}", two_cube_sums(4104));
}

#[cfg(test)]
mod tests {
    use super::two_cube_sums;

    #[test]
    fn example1() {
        assert_eq!(two_cube_sums(4104), vec![1729, 4104]);
    }

    #[test]
    fn example2() {
        assert_eq!(two_cube_sums(578), Vec::<i64>::new());
    }
}
