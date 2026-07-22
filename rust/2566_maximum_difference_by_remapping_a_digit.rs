/// LeetCode #2566 - Maximum Difference by Remapping a Digit
fn min_max_difference(num: i32) -> i32 {
    let s = num.to_string();
    let first = s.chars().next().unwrap();
    let mi: i32 = s
        .chars()
        .map(|c| if c == first { '0' } else { c })
        .collect::<String>()
        .parse()
        .unwrap();

    for c in s.chars() {
        if c != '9' {
            let mx: i32 = s
                .chars()
                .map(|ch| if ch == c { '9' } else { ch })
                .collect::<String>()
                .parse()
                .unwrap();
            return mx - mi;
        }
    }
    num - mi
}

fn main() {
    println!("{}", min_max_difference(11891));
}

#[cfg(test)]
mod tests {
    use super::min_max_difference;

    #[test]
    fn example_one() {
        assert_eq!(min_max_difference(11891), 99009);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_max_difference(90), 99);
    }
}
