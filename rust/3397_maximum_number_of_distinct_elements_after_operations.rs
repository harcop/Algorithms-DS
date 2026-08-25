/// LeetCode #3397 - Maximum Number of Distinct Elements After Operations
fn max_distinct_elements(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();
    let mut ans = 0;
    let mut pre = i64::MIN / 4;
    let k = k as i64;
    for x in nums {
        let x = x as i64;
        let cur = (x + k).min((x - k).max(pre + 1));
        if cur > pre {
            ans += 1;
            pre = cur;
        }
    }
    ans
}

fn main() {
    println!("{}", max_distinct_elements(vec![1, 2, 2, 3, 3, 4], 2));
}

#[cfg(test)]
mod tests {
    use super::max_distinct_elements;

    #[test]
    fn example1() {
        assert_eq!(max_distinct_elements(vec![1, 2, 2, 3, 3, 4], 2), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(max_distinct_elements(vec![4, 4, 4, 4], 1), 3);
    }
}
