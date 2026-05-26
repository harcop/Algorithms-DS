/// LeetCode #1432 - Max Difference You Can Get From Changing An Integer
fn max_diff(num: i32) -> i32 {
    let s = num.to_string();
    let chars: Vec<char> = s.chars().collect();

    let mut max_s = s.clone();
    let mut max_rep = chars[0];
    for &c in &chars {
        if c != '9' {
            max_rep = c;
            break;
        }
    }
    max_s = max_s.replace(max_rep, "9");

    let mut min_s = s.clone();
    if chars[0] != '1' {
        min_s = min_s.replace(chars[0], "1");
    } else {
        for &c in chars.iter().skip(1) {
            if c != '0' && c != chars[0] {
                min_s = min_s.replace(c, "0");
                break;
            }
        }
    }

    max_s.parse::<i32>().unwrap() - min_s.parse::<i32>().unwrap()
}

fn main() {
    println!("{}", max_diff(555));
}

#[cfg(test)]
mod tests {
    use super::max_diff;

    #[test]
    fn example_one() {
        assert_eq!(max_diff(555), 888);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_diff(9), 8);
    }
}
