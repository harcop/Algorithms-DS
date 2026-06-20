/// LeetCode #1995 - Count Special Quadruplets
fn count_quadruplets(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ans = 0i32;
    for a in 0..n - 3 {
        for b in a + 1..n - 2 {
            for c in b + 1..n - 1 {
                for d in c + 1..n {
                    if nums[a] + nums[b] + nums[c] == nums[d] {
                        ans += 1;
                    }
                }
            }
        }
    }
    ans
}

fn main() {
    println!("{}", count_quadruplets(vec![1, 2, 3, 6]));
}

#[cfg(test)]
mod tests {
    use super::count_quadruplets;

    #[test]
    fn example_one() {
        assert_eq!(count_quadruplets(vec![1, 2, 3, 6]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_quadruplets(vec![3, 3, 6, 4, 5]), 0);
    }
}
