/// LeetCode #1154 - Day of the Year
fn day_of_year(date: String) -> i32 {
    let parts: Vec<i32> = date.split('-').map(|s| s.parse().unwrap()).collect();
    let y = parts[0];
    let m = parts[1];
    let d = parts[2];
    let days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let leap = y % 400 == 0 || (y % 4 == 0 && y % 100 != 0);
    let mut ans = d;
    for i in 0..(m - 1) as usize {
        ans += days[i];
        if i == 1 && leap {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", day_of_year("2019-02-10".to_string()));
}

#[cfg(test)]
mod tests {
    use super::day_of_year;

    #[test]
    fn example_one() {
        assert_eq!(day_of_year("2019-02-10".to_string()), 41);
    }

    #[test]
    fn example_two() {
        assert_eq!(day_of_year("2019-06-10".to_string()), 161);
    }
}
