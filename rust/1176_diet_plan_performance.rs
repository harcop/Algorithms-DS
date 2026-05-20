/// LeetCode #1176 - Diet Plan Performance
fn diet_plan_performance(calories: Vec<i32>, k: i32, lower: i32, upper: i32) -> i32 {
    let k = k as usize;
    if calories.len() < k {
        return 0;
    }
    let mut window: i32 = calories[..k].iter().sum();
    let mut score = 0i32;
    if window < lower {
        score -= 1;
    } else if window > upper {
        score += 1;
    }
    for i in k..calories.len() {
        window += calories[i] - calories[i - k];
        if window < lower {
            score -= 1;
        } else if window > upper {
            score += 1;
        }
    }
    score
}

fn main() {
    println!(
        "{}",
        diet_plan_performance(vec![1, 2, 3, 4, 5], 1, 3, 3)
    );
}

#[cfg(test)]
mod tests {
    use super::diet_plan_performance;

    #[test]
    fn example_one() {
        assert_eq!(diet_plan_performance(vec![1, 2, 3, 4, 5], 1, 3, 3), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(diet_plan_performance(vec![3, 2], 2, 0, 1), 1);
    }
}
