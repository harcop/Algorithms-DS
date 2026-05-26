/// LeetCode #1404 - Number Of Steps To Reduce A Number In Binary Representation To One
fn num_steps(s: String) -> i32 {
    let mut s = s;
    let mut steps = 0i32;
    while s != "1" {
        if s.ends_with('0') {
            s.pop();
            steps += 1;
        } else {
            let n = u128::from_str_radix(&s, 2).unwrap() + 1;
            s = format!("{:b}", n);
            steps += 1;
        }
    }
    steps
}

fn main() {
    println!("{}", num_steps("1101".into()));
}

#[cfg(test)]
mod tests {
    use super::num_steps;

    #[test]
    fn example_one() {
        assert_eq!(num_steps("1101".into()), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_steps("10".into()), 1);
    }
}

