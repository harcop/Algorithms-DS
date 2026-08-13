/// LeetCode #3185 - Count Pairs That Form a Complete Day II
fn count_complete_day_pairs(hours: Vec<i32>) -> i64 {
    let mut cnt = [0i64; 24];
    let mut ans = 0i64;
    for x in hours {
        let r = (x % 24) as usize;
        ans += cnt[(24 - r) % 24];
        cnt[r] += 1;
    }
    ans
}

fn main() {
    println!("{}", count_complete_day_pairs(vec![12, 12, 30, 24, 24]));
}

#[cfg(test)]
mod tests {
    use super::count_complete_day_pairs;

    #[test]
    fn example1() {
        assert_eq!(count_complete_day_pairs(vec![12, 12, 30, 24, 24]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(count_complete_day_pairs(vec![72, 48, 24, 3]), 3);
    }
}
