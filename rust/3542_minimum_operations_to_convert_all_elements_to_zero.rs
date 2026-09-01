/// LeetCode #3542 - Minimum Operations to Convert All Elements to Zero
fn min_operations(nums: Vec<i32>) -> i32 {
    let mut stk = Vec::new();
    let mut ans = 0;
    for x in nums {
        while stk.last().is_some_and(|&y| y > x) {
            ans += 1;
            stk.pop();
        }
        if x != 0 && (stk.is_empty() || *stk.last().unwrap() != x) {
            stk.push(x);
        }
    }
    ans + stk.len() as i32
}

fn main() {
    println!("{}", min_operations(vec![0, 2]));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(vec![0, 2]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![3, 1, 2, 1]), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(min_operations(vec![1, 2, 1, 2, 1, 2]), 4);
    }
}
