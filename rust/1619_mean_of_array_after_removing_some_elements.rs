/// LeetCode #1619 - Mean Of Array After Removing Some Elements
fn trim_mean(arr: Vec<i32>) -> f64 {
    let mut a = arr;
    a.sort_unstable();
    let n = a.len();
    let k = n / 20;
    let sum: i64 = a[k..n - k].iter().map(|&x| x as i64).sum();
    sum as f64 / (n - 2 * k) as f64
}

fn main() {
    println!("{}", trim_mean(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]));
}

#[cfg(test)]
mod tests {
    use super::trim_mean;

    #[test]
    fn example_one() {
        let t = trim_mean(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
        assert!((t - 10.5).abs() < 1e-5);
    }
}
