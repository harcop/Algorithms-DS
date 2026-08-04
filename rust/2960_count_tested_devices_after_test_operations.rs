/// LeetCode #2960 - Count Tested Devices After Test Operations
fn count_tested_devices(battery_percentages: Vec<i32>) -> i32 {
    let mut ans = 0;
    for x in battery_percentages {
        if x > ans {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", count_tested_devices(vec![1, 1, 2, 1, 3]));
}

#[cfg(test)]
mod tests {
    use super::count_tested_devices;

    #[test]
    fn example_one() {
        assert_eq!(count_tested_devices(vec![1, 1, 2, 1, 3]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_tested_devices(vec![0, 1, 2]), 2);
    }
}
