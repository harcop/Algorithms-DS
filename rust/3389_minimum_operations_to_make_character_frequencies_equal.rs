/// LeetCode #3389 - Minimum Operations to Make Character Frequencies Equal
fn make_string_good(s: String) -> i32 {
    let mut count = [0i32; 26];
    for b in s.bytes() {
        count[(b - b'a') as usize] += 1;
    }
    let mx = *count.iter().max().unwrap();
    let mut ans = s.len() as i32;
    for target in 1..=mx {
        ans = ans.min(min_ops(&count, target));
    }
    ans
}

fn min_ops(count: &[i32; 26], target: i32) -> i32 {
    let mut dp = [0i32; 27];
    for i in (0..26).rev() {
        let delete_all = count[i];
        let to_target = (target - count[i]).abs();
        dp[i] = delete_all.min(to_target) + dp[i + 1];
        if i + 1 < 26 && count[i + 1] < target {
            let next_deficit = target - count[i + 1];
            let need_to_change = if count[i] > target {
                count[i] - target
            } else {
                count[i]
            };
            let change_to_target = need_to_change.max(next_deficit);
            dp[i] = dp[i].min(change_to_target + dp[i + 2]);
        }
    }
    dp[0]
}

fn main() {
    println!("{}", make_string_good("acab".into()));
}

#[cfg(test)]
mod tests {
    use super::make_string_good;

    #[test]
    fn example1() {
        assert_eq!(make_string_good("acab".into()), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(make_string_good("wddw".into()), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(make_string_good("aaabc".into()), 2);
    }
}
