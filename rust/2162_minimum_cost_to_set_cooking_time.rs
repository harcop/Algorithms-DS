/// LeetCode #2162 - Minimum Cost to Set Cooking Time
fn min_cost_set_time(start_at: i32, move_cost: i32, push_cost: i32, target_seconds: i32) -> i32 {
    fn cost(start_at: i32, move_cost: i32, push_cost: i32, minutes: i32, seconds: i32) -> i32 {
        if !(0..=99).contains(&minutes) || !(0..=99).contains(&seconds) {
            return i32::MAX / 2;
        }

        let value = format!("{}{:02}", minutes, seconds);
        let digits: Vec<i32> = value
            .trim_start_matches('0')
            .bytes()
            .map(|b| (b - b'0') as i32)
            .collect();
        let mut cur = start_at;
        let mut ans = 0i32;
        for d in digits {
            if cur != d {
                ans += move_cost;
                cur = d;
            }
            ans += push_cost;
        }
        ans
    }

    let minutes = target_seconds / 60;
    let seconds = target_seconds % 60;
    cost(start_at, move_cost, push_cost, minutes, seconds).min(cost(
        start_at,
        move_cost,
        push_cost,
        minutes - 1,
        seconds + 60,
    ))
}

fn main() {
    println!("{}", min_cost_set_time(1, 2, 1, 600));
}

#[cfg(test)]
mod tests {
    use super::min_cost_set_time;

    #[test]
    fn example_one() {
        assert_eq!(min_cost_set_time(1, 2, 1, 600), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_cost_set_time(0, 1, 2, 76), 6);
    }
}
