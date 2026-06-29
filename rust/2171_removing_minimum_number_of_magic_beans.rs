/// LeetCode #2171 - Removing Minimum Number of Magic Beans
fn minimum_removal(mut beans: Vec<i32>) -> i64 {
    beans.sort_unstable();
    let total: i64 = beans.iter().map(|&x| x as i64).sum();
    let n = beans.len() as i64;
    let mut keep = 0i64;
    for (i, &x) in beans.iter().enumerate() {
        keep = keep.max((n - i as i64) * x as i64);
    }
    total - keep
}

fn main() {
    println!("{}", minimum_removal(vec![4, 1, 6, 5]));
}

#[cfg(test)]
mod tests {
    use super::minimum_removal;

    #[test]
    fn example_one() {
        assert_eq!(minimum_removal(vec![4, 1, 6, 5]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_removal(vec![2, 10, 3, 2]), 7);
    }
}
