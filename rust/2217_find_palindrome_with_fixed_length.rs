/// LeetCode #2217 - Find Palindrome With Fixed Length
fn reverse_num(mut num: i64) -> i64 {
    let mut res = 0i64;
    while num > 0 {
        res = res * 10 + num % 10;
        num /= 10;
    }
    res
}

fn kth_palindrome(queries: Vec<i32>, int_length: i32) -> Vec<i64> {
    let int_length = int_length as i64;
    let start = 10i64.pow(((int_length + 1) / 2 - 1) as u32);
    let end = 10i64.pow(((int_length + 1) / 2) as u32);
    let mul = 10i64.pow((int_length / 2) as u32);

    queries
        .into_iter()
        .map(|q| {
            let q = q as i64;
            if q == 0 || start + q > end {
                -1
            } else {
                let prefix = start + q - 1;
                let suffix_src = if int_length % 2 == 0 {
                    prefix
                } else {
                    prefix / 10
                };
                prefix * mul + reverse_num(suffix_src)
            }
        })
        .collect()
}

fn main() {
    println!("{:?}", kth_palindrome(vec![1, 2, 3, 4, 90, 0], 2));
}

#[cfg(test)]
mod tests {
    use super::kth_palindrome;

    #[test]
    fn example_one() {
        assert_eq!(
            kth_palindrome(vec![1, 2, 3, 4, 90, 0], 2),
            vec![11, 22, 33, 44, -1, -1]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(kth_palindrome(vec![2, 4, 6], 4), vec![1111, 1331, 1551]);
    }
}
