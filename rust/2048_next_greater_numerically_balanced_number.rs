/// LeetCode #2048 - Next Greater Numerically Balanced Number
fn next_beautiful_number(n: i32) -> i32 {
    fn balanced(x: i32) -> bool {
        let mut cnt = [0i32; 10];
        let mut y = x;
        while y > 0 {
            cnt[(y % 10) as usize] += 1;
            y /= 10;
        }
        (0..10).all(|i| cnt[i] == 0 || cnt[i] == i as i32)
    }

    let mut x = n + 1;
    loop {
        if balanced(x) {
            return x;
        }
        x += 1;
    }
}

fn main() {
    println!("{}", next_beautiful_number(1));
}

#[cfg(test)]
mod tests {
    use super::next_beautiful_number;

    #[test]
    fn example_one() {
        assert_eq!(next_beautiful_number(1), 22);
    }

    #[test]
    fn example_two() {
        assert_eq!(next_beautiful_number(1000), 1333);
    }

    #[test]
    fn example_three() {
        assert_eq!(next_beautiful_number(3000), 3133);
    }
}
