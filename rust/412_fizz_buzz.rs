/// LeetCode #412 - Fizz Buzz
fn fizz_buzz(n: i32) -> Vec<String> {
    (1..=n)
        .map(|i| {
            if i % 15 == 0 {
                "FizzBuzz".to_string()
            } else if i % 3 == 0 {
                "Fizz".to_string()
            } else if i % 5 == 0 {
                "Buzz".to_string()
            } else {
                i.to_string()
            }
        })
        .collect()
}

fn main() {
    println!("{:?}", fizz_buzz(15));
}

#[cfg(test)]
mod tests {
    use super::fizz_buzz;

    #[test]
    fn example_one() {
        assert_eq!(fizz_buzz(5), vec!["1","2","Fizz","4","Buzz"]);
    }
}
