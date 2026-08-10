/// LeetCode #3113 - Find the Number of Subarrays Where Boundary Elements Are Maximum
fn number_of_subarrays(nums: Vec<i32>) -> i64 {
    let mut stk: Vec<(i32, i64)> = Vec::new();
    let mut ans = 0i64;
    for x in nums {
        while stk.last().map_or(false, |&(v, _)| v < x) {
            stk.pop();
        }
        if stk.is_empty() || stk.last().unwrap().0 > x {
            stk.push((x, 1));
        } else {
            stk.last_mut().unwrap().1 += 1;
        }
        ans += stk.last().unwrap().1;
    }
    ans
}

fn main() {
    println!("{}", number_of_subarrays(vec![1, 4, 3, 3, 2]));
}

#[cfg(test)]
mod tests {
    use super::number_of_subarrays;

    #[test]
    fn example1() {
        assert_eq!(number_of_subarrays(vec![1, 4, 3, 3, 2]), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(number_of_subarrays(vec![3, 3, 3]), 6);
    }

    #[test]
    fn example3() {
        assert_eq!(number_of_subarrays(vec![1]), 1);
    }
}
