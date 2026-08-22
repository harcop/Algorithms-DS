/// LeetCode #3354 - Make Array Elements Equal to Zero
fn count_valid_selections(nums: Vec<i32>) -> i32 {
    let s: i32 = nums.iter().sum();
    let mut ans = 0;
    let mut l = 0;
    for x in nums {
        if x != 0 {
            l += x;
        } else if l * 2 == s {
            ans += 2;
        } else if (l * 2 - s).abs() == 1 {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", count_valid_selections(vec![1, 0, 2, 0, 3]));
}

#[cfg(test)]
mod tests {
    use super::count_valid_selections;

    #[test]
    fn example1() {
        assert_eq!(count_valid_selections(vec![1, 0, 2, 0, 3]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(count_valid_selections(vec![2, 3, 4, 0, 4, 1, 0]), 0);
    }
}
