/// LeetCode #1215 - Stepping Numbers
fn count_stepping_numbers(low: i64, high: i64) -> i32 {
    if high == 0 {
        return 1;
    }
    let mut ans = 0i32;
    let mut q = std::collections::VecDeque::new();
    if low == 0 {
        ans += 1;
    }
    for d in 1..=9 {
        q.push_back(d as i64);
    }
    while let Some(x) = q.pop_front() {
        if x > high {
            continue;
        }
        if x >= low {
            ans += 1;
        }
        let last = (x % 10) as i32;
        if last > 0 {
            q.push_back(x * 10 + (last - 1) as i64);
        }
        if last < 9 {
            q.push_back(x * 10 + (last + 1) as i64);
        }
    }
    ans
}

fn main() {
    println!("{}", count_stepping_numbers(0, 21));
}

#[cfg(test)]
mod tests {
    use super::count_stepping_numbers;

    #[test]
    fn example_one() {
        assert_eq!(count_stepping_numbers(0, 21), 13);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_stepping_numbers(10, 15), 2);
    }
}
