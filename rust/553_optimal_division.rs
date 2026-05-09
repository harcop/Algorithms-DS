/// LeetCode #553 - Optimal Division
fn optimal_division(nums: Vec<i32>) -> String {
    if nums.len() == 1 {
        return nums[0].to_string();
    }
    if nums.len() == 2 {
        return format!("{}/{}", nums[0], nums[1]);
    }
    let rest: String = nums[1..]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("/");
    format!("{}/({})", nums[0], rest)
}

fn main() {
    println!("{}", optimal_division(vec![1000, 100, 10, 2]));
}

#[cfg(test)]
mod tests {
    use super::optimal_division;

    #[test]
    fn example_one() {
        assert_eq!(optimal_division(vec![1000, 100, 10, 2]), "1000/(100/10/2)");
    }
}
