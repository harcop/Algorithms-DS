/// LeetCode #4054 - Count Shadow Pairs I
fn count_shadow_pairs(nums: Vec<i32>) -> i64 {
    let mut stack: Vec<(i32, i64)> = Vec::new();
    let mut active = 0i64;
    let mut ans = 0i64;
    for x in nums {
        while stack.last().is_some_and(|&(v, _)| v > x) {
            active -= stack.pop().unwrap().1;
        }
        ans += active;
        if stack.last().is_some_and(|&(v, _)| v == x) {
            let freq = stack.last_mut().unwrap();
            ans -= freq.1;
            freq.1 += 1;
        } else {
            stack.push((x, 1));
        }
        active += 1;
    }
    ans
}

fn main() {
    println!("{}", count_shadow_pairs(vec![3, 1, 4, 1, 5]));
}

#[cfg(test)]
mod tests {
    use super::count_shadow_pairs;

    #[test]
    fn example1() {
        assert_eq!(count_shadow_pairs(vec![3, 1, 4, 1, 5]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(count_shadow_pairs(vec![6, 7, 6, 6, 7]), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(count_shadow_pairs(vec![1, 2, 3, 4]), 6);
    }
}
