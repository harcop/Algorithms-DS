/// LeetCode #3100 - Water Bottles II
fn max_bottles_drunk(mut num_bottles: i32, mut num_exchange: i32) -> i32 {
    let mut ans = num_bottles;
    while num_bottles >= num_exchange {
        num_bottles -= num_exchange;
        num_exchange += 1;
        ans += 1;
        num_bottles += 1;
    }
    ans
}

fn main() {
    println!("{}", max_bottles_drunk(13, 6));
}

#[cfg(test)]
mod tests {
    use super::max_bottles_drunk;

    #[test]
    fn example1() {
        assert_eq!(max_bottles_drunk(13, 6), 15);
    }

    #[test]
    fn example2() {
        assert_eq!(max_bottles_drunk(10, 3), 13);
    }
}
