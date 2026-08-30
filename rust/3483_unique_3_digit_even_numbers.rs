/// LeetCode #3483 - Unique 3-Digit Even Numbers
fn total_numbers(digits: Vec<i32>) -> i32 {
    let mut s = std::collections::HashSet::new();
    for (i, &a) in digits.iter().enumerate() {
        if a & 1 != 0 {
            continue;
        }
        for (j, &b) in digits.iter().enumerate() {
            if i == j {
                continue;
            }
            for (k, &c) in digits.iter().enumerate() {
                if c == 0 || k == i || k == j {
                    continue;
                }
                s.insert(c * 100 + b * 10 + a);
            }
        }
    }
    s.len() as i32
}

fn main() {
    println!("{}", total_numbers(vec![1, 2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::total_numbers;

    #[test]
    fn example1() {
        assert_eq!(total_numbers(vec![1, 2, 3, 4]), 12);
    }

    #[test]
    fn example2() {
        assert_eq!(total_numbers(vec![0, 2, 2]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(total_numbers(vec![6, 6, 6]), 1);
    }

    #[test]
    fn example4() {
        assert_eq!(total_numbers(vec![1, 3, 5]), 0);
    }
}
