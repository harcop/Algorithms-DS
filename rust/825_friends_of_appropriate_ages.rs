/// LeetCode #825 - Friends of Appropriate Ages
fn num_friend_requests(ages: Vec<i32>) -> i32 {
    let mut cnt = [0i32; 121];
    for a in ages {
        cnt[a as usize] += 1;
    }
    let mut ans = 0;
    for age_a in 15..=120 {
        if cnt[age_a] == 0 {
            continue;
        }
        let min_b = (age_a as f64 / 2.0 + 7.0) as i32 + 1;
        for age_b in (min_b as usize)..=age_a {
            ans += cnt[age_a] * cnt[age_b];
            if age_a == age_b {
                ans -= cnt[age_a];
            }
        }
    }
    ans
}

fn main() {
    println!("{}", num_friend_requests(vec![16, 16]));
}

#[cfg(test)]
mod tests {
    use super::num_friend_requests;

    #[test]
    fn example_one() {
        assert_eq!(num_friend_requests(vec![16, 16]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_friend_requests(vec![16, 17, 18]), 2);
    }
}
