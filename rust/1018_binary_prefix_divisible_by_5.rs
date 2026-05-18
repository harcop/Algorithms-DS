/// LeetCode #1018 - Binary Prefix Divisible By 5
fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
    let mut rem = 0i32;
    nums.into_iter()
        .map(|b| {
            rem = (rem * 2 + b) % 5;
            rem == 0
        })
        .collect()
}

fn main() {
    println!("{:?}", prefixes_div_by5(vec![0, 1, 1]));
}

#[cfg(test)]
mod tests {
    use super::prefixes_div_by5;

    #[test]
    fn example_one() {
        assert_eq!(prefixes_div_by5(vec![0, 1, 1]), vec![true, false, false]);
    }

    #[test]
    fn example_two() {
        assert_eq!(prefixes_div_by5(vec![1, 1, 1]), vec![false, false, false]);
    }
}
