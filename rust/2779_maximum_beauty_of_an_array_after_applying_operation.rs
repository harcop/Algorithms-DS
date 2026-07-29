/// LeetCode #2779 - Maximum Beauty of an Array After Applying Operation
fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
    let max_val = *nums.iter().max().unwrap();
    let m = (max_val + k * 2 + 2) as usize;
    let mut d = vec![0i32; m];
    for &x in &nums {
        d[x as usize] += 1;
        d[(x + k * 2 + 1) as usize] -= 1;
    }
    let mut ans = 0;
    let mut s = 0;
    for &x in &d {
        s += x;
        ans = ans.max(s);
    }
    ans
}

fn main() {
    println!("{}", maximum_beauty(vec![4, 6, 1, 2], 2));
}

#[cfg(test)]
mod tests {
    use super::maximum_beauty;

    #[test]
    fn example_one() {
        assert_eq!(maximum_beauty(vec![4, 6, 1, 2], 2), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_beauty(vec![1, 1, 1, 1], 10), 4);
    }
}
