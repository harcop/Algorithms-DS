/// LeetCode #3644 - Maximum K to Sort a Permutation
fn sort_permutation(nums: Vec<i32>) -> i32 {
    let mut ans = -1i32;
    for (i, &x) in nums.iter().enumerate() {
        if i as i32 != x {
            ans &= x;
        }
    }
    ans.max(0)
}

fn main() {
    println!("{}", sort_permutation(vec![0, 3, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::sort_permutation;

    #[test]
    fn example1() {
        assert_eq!(sort_permutation(vec![0, 3, 2, 1]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(sort_permutation(vec![0, 1, 3, 2]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(sort_permutation(vec![3, 2, 1, 0]), 0);
    }
}
