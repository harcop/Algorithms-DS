/// LeetCode #165 - Compare Version Numbers
fn compare_version(version1: String, version2: String) -> i32 {
    let a: Vec<i32> = version1.split('.').map(|s| s.parse().unwrap_or(0)).collect();
    let b: Vec<i32> = version2.split('.').map(|s| s.parse().unwrap_or(0)).collect();
    let n = a.len().max(b.len());
    for i in 0..n {
        let x = *a.get(i).unwrap_or(&0);
        let y = *b.get(i).unwrap_or(&0);
        match x.cmp(&y) {
            std::cmp::Ordering::Less => return -1,
            std::cmp::Ordering::Greater => return 1,
            std::cmp::Ordering::Equal => {}
        }
    }
    0
}

fn main() {
    println!("{}", compare_version("1.01".into(), "1.001".into()));
}

#[cfg(test)]
mod tests {
    use super::compare_version;

    #[test]
    fn example_one() {
        assert_eq!(compare_version("1.01".into(), "1.001".into()), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(compare_version("1.0".into(), "1.0.0".into()), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(compare_version("0.1".into(), "1.1".into()), -1);
    }
}
