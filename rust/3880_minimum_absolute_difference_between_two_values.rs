/// LeetCode #3880 - Minimum Absolute Difference Between Two Values
fn min_absolute_difference(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i32;
    let mut ans = n + 1;
    let mut last = [-(n + 1); 3];
    for (i, &x) in nums.iter().enumerate() {
        if x != 0 {
            ans = ans.min(i as i32 - last[(3 - x) as usize]);
            last[x as usize] = i as i32;
        }
    }
    if ans > n {
        -1
    } else {
        ans
    }
}

fn main() {
    println!("{}", min_absolute_difference(vec![1, 0, 0, 2, 0, 1]));
}

#[cfg(test)]
mod tests {
    use super::min_absolute_difference;

    #[test]
    fn example1() {
        assert_eq!(min_absolute_difference(vec![1, 0, 0, 2, 0, 1]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(min_absolute_difference(vec![1, 0, 1, 0]), -1);
    }
}
