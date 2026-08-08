/// LeetCode #3075 - Maximize Happiness of Selected Children
fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
    happiness.sort_unstable_by(|a, b| b.cmp(a));
    let mut ans = 0i64;
    for i in 0..k as usize {
        let h = (happiness[i] as i64 - i as i64).max(0);
        ans += h;
    }
    ans
}

fn main() {
    println!("{}", maximum_happiness_sum(vec![1, 2, 3], 2));
}

#[cfg(test)]
mod tests {
    use super::maximum_happiness_sum;

    #[test]
    fn example1() {
        assert_eq!(maximum_happiness_sum(vec![1, 2, 3], 2), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_happiness_sum(vec![1, 1, 1, 1], 2), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(maximum_happiness_sum(vec![2, 3, 4, 5], 1), 5);
    }
}
