/// LeetCode #1491 - Average Salary Excluding The Minimum And Maximum Salary
fn average(salary: Vec<i32>) -> f64 {
    let mn = *salary.iter().min().unwrap();
    let mx = *salary.iter().max().unwrap();
    let sum: i64 = salary.iter().map(|&x| x as i64).sum();
    (sum - mn as i64 - mx as i64) as f64 / (salary.len() as f64 - 2.0)
}
fn main() { println!("{}", average(vec![4000,3000,1000,2000])); }
#[cfg(test)]
mod tests {
    use super::average;
    #[test]
    fn example_one() { assert!((average(vec![4000,3000,1000,2000]) - 2500.0).abs() < 1e-5); }
    #[test]
    fn example_two() { assert!((average(vec![1000,2000,3000]) - 2000.0).abs() < 1e-5); }
}