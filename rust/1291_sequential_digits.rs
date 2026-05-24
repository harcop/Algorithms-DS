/// LeetCode #1291 - Sequential Digits
fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
    let mut ans = vec![];
    for start in 1..=9 {
        let mut num = start;
        let mut next = start + 1;
        while next <= 9 {
            num = num * 10 + next;
            if num > high {
                break;
            }
            if num >= low {
                ans.push(num);
            }
            next += 1;
        }
    }
    ans.sort_unstable();
    ans
}

fn main() {
    println!("{:?}", sequential_digits(100, 300));
}

#[cfg(test)]
mod tests {
    use super::sequential_digits;

    #[test]
    fn example_one() {
        assert_eq!(sequential_digits(100, 300), vec![123, 234]);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            sequential_digits(1000, 13000),
            vec![1234, 2345, 3456, 4567, 5678, 6789, 12345]
        );
    }
}
