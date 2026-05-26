/// LeetCode #1414 - Find The Minimum Number Of Fibonacci Numbers Whose Sum Is K
fn find_min_fibonacci_numbers(k: i32) -> i32 {
    let mut fib = vec![1i32, 1];
    while *fib.last().unwrap() < k {
        let n = fib.len();
        fib.push(fib[n - 1] + fib[n - 2]);
    }
    let mut rem = k;
    let mut ans = 0;
    for &f in fib.iter().rev() {
        if f <= rem {
            rem -= f;
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", find_min_fibonacci_numbers(7));
}

#[cfg(test)]
mod tests {
    use super::find_min_fibonacci_numbers;

    #[test]
    fn example_one() {
        assert_eq!(find_min_fibonacci_numbers(7), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_min_fibonacci_numbers(10), 2);
    }
}

