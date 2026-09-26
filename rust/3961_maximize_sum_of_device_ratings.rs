/// LeetCode #3961 - Maximize Sum of Device Ratings
fn max_ratings(mut units: Vec<Vec<i32>>) -> i64 {
    let n = units[0].len();
    if n == 1 {
        return units.iter().map(|x| x[0] as i64).sum();
    }
    let mut ans = 0i64;
    let mut mn = i32::MAX;
    let mut mn2 = i32::MAX;
    for x in &mut units {
        x.sort_unstable();
        ans += x[1] as i64;
        mn2 = mn2.min(x[1]);
        mn = mn.min(x[0]);
    }
    ans - (mn2 - mn) as i64
}

fn main() {
    println!("{}", max_ratings(vec![vec![1, 3], vec![2, 2]]));
}

#[cfg(test)]
mod tests {
    use super::max_ratings;

    #[test]
    fn example1() {
        assert_eq!(max_ratings(vec![vec![1, 3], vec![2, 2]]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(max_ratings(vec![vec![1, 2, 3], vec![4, 5, 6]]), 6);
    }

    #[test]
    fn example3() {
        assert_eq!(max_ratings(vec![vec![5, 5, 5], vec![1, 1, 1]]), 6);
    }
}
