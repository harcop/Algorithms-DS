/// LeetCode #2289 - Steps to Make Array Non-decreasing
fn total_steps(nums: Vec<i32>) -> i32 {
    let mut ans = 0i32;
    let mut stack: Vec<(i32, i32)> = Vec::new(); // (value, steps)

    for &x in nums.iter().rev() {
        let mut steps = 0i32;
        while let Some(&(v, s)) = stack.last() {
            if x > v {
                steps = steps.max(s + 1);
                stack.pop();
            } else {
                break;
            }
        }
        ans = ans.max(steps);
        stack.push((x, steps));
    }
    ans
}

fn main() {
    println!("{}", total_steps(vec![5, 3, 4, 4, 7, 3, 6, 11, 8, 5, 11]));
}

#[cfg(test)]
mod tests {
    use super::total_steps;

    #[test]
    fn example_one() {
        assert_eq!(total_steps(vec![5, 3, 4, 4, 7, 3, 6, 11, 8, 5, 11]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(total_steps(vec![4, 5, 7, 7, 13]), 0);
    }
}

