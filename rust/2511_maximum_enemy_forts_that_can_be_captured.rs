/// LeetCode #2511 - Maximum Enemy Forts That Can Be Captured
fn capture_forts(forts: Vec<i32>) -> i32 {
    let n = forts.len();
    let mut ans = 0;
    let mut i = 0;
    while i < n {
        let mut j = i + 1;
        if forts[i] != 0 {
            while j < n && forts[j] == 0 {
                j += 1;
            }
            if j < n && forts[i] + forts[j] == 0 {
                ans = ans.max((j - i - 1) as i32);
            }
        }
        i = j;
    }
    ans
}

fn main() {
    println!("{}", capture_forts(vec![1, 0, 0, -1, 0, 0, 0, 0, 1]));
}

#[cfg(test)]
mod tests {
    use super::capture_forts;

    #[test]
    fn example_one() {
        assert_eq!(capture_forts(vec![1, 0, 0, -1, 0, 0, 0, 0, 1]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(capture_forts(vec![0, 0, 1, -1]), 0);
    }
}
