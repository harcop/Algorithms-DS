/// LeetCode #179 - Largest Number
fn largest_number(nums: Vec<i32>) -> String {
    let mut s: Vec<String> = nums.into_iter().map(|x| x.to_string()).collect();
    s.sort_by(|a, b| (b.clone() + a).cmp(&(a.clone() + b)));
    if s[0] == "0" {
        return "0".into();
    }
    s.concat()
}

fn main() {
    println!("{}", largest_number(vec![10, 2]));
}

#[cfg(test)]
mod tests {
    use super::largest_number;

    #[test]
    fn example_one() {
        assert_eq!(largest_number(vec![10, 2]), "210");
    }

    #[test]
    fn example_two() {
        assert_eq!(largest_number(vec![3, 30, 34, 5, 9]), "9534330");
    }
}
