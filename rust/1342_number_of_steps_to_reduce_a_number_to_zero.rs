/// LeetCode #1342 - Number Of Steps To Reduce A Number To Zero

fn get_steps(num: i32) -> i32 {
    let mut n = num;
    let mut steps = 0;
    while n > 0 {
        steps += 1;
        if n % 2 == 0 {
            n /= 2;
        } else {
            n -= 1;
        }
    }
    steps
}

fn main() {
    println!("{}", get_steps(14));
}

#[cfg(test)]
mod tests {
    use super::get_steps;

    #[test]
    fn example_one() {
        assert_eq!(get_steps(14), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(get_steps(8), 4);
    }
}
