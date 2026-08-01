/// LeetCode #2860 - Happy Students
fn count_ways(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    let mut answer = 0;

    for selected in 0..=n {
        if selected > 0 && nums[selected - 1] >= selected as i32 {
            continue;
        }
        if selected < n && nums[selected] <= selected as i32 {
            continue;
        }
        answer += 1;
    }
    answer
}

fn main() {
    println!("{}", count_ways(vec![6, 0, 3, 3, 6, 7, 2, 7]));
}

#[cfg(test)]
mod tests {
    use super::count_ways;

    #[test]
    fn example_one() {
        assert_eq!(count_ways(vec![1, 1]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_ways(vec![6, 0, 3, 3, 6, 7, 2, 7]), 3);
    }
}
