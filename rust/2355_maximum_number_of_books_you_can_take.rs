/// LeetCode #2355 - Maximum Number of Books You Can Take
fn maximum_books(books: Vec<i32>) -> i64 {
    let n = books.len();
    let nums: Vec<i32> = books.iter().enumerate().map(|(i, &v)| v - i as i32).collect();
    let mut left = vec![-1i32; n];
    let mut stk: Vec<usize> = Vec::new();
    for i in 0..n {
        while let Some(&top) = stk.last() {
            if nums[top] >= nums[i] {
                stk.pop();
            } else {
                break;
            }
        }
        if let Some(&top) = stk.last() {
            left[i] = top as i32;
        }
        stk.push(i);
    }

    let mut dp = vec![0i64; n];
    let mut ans = 0i64;
    for i in 0..n {
        let v = books[i] as i64;
        let j = left[i];
        let cnt = v.min((i as i32 - j) as i64);
        let u = v - cnt + 1;
        let s = (u + v) * cnt / 2;
        dp[i] = s + if j == -1 { 0 } else { dp[j as usize] };
        ans = ans.max(dp[i]);
    }
    ans
}

fn main() {
    println!("{}", maximum_books(vec![8, 5, 2, 7, 9]));
}

#[cfg(test)]
mod tests {
    use super::maximum_books;

    #[test]
    fn example_one() {
        assert_eq!(maximum_books(vec![8, 5, 2, 7, 9]), 19);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_books(vec![7, 0, 3, 4, 5]), 12);
    }

    #[test]
    fn example_three() {
        assert_eq!(
            maximum_books(vec![8, 2, 3, 7, 3, 4, 0, 1, 4, 3]),
            13
        );
    }
}
