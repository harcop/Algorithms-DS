/// LeetCode #3023 - Find Pattern in Infinite Stream I (Rust analogue)
fn find_pattern(stream: Vec<i32>, pattern: Vec<i32>) -> i32 {
    let n = stream.len();
    let m = pattern.len();
    if m == 0 || m > n {
        return -1;
    }
    for i in 0..=n - m {
        if stream[i..i + m] == pattern[..] {
            return i as i32;
        }
    }
    -1
}

fn main() {
    let stream = vec![1, 1, 1, 0, 1, 1, 1];
    let pattern = vec![0, 1];
    println!("{}", find_pattern(stream, pattern));
}

#[cfg(test)]
mod tests {
    use super::find_pattern;

    #[test]
    fn example1() {
        assert_eq!(
            find_pattern(vec![1, 1, 1, 0, 1, 1, 1], vec![0, 1]),
            3
        );
    }

    #[test]
    fn example2() {
        assert_eq!(find_pattern(vec![0, 0, 0, 0], vec![0]), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(
            find_pattern(vec![1, 0, 1, 1, 0, 1, 1, 0, 1], vec![1, 1, 0, 1]),
            2
        );
    }
}
