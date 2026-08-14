/// LeetCode #3205 - Maximum Array Hopping Score I
fn max_score(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut f = vec![0i32; n];
    for i in (0..n - 1).rev() {
        let mut best = 0;
        for j in i + 1..n {
            best = best.max((j - i) as i32 * nums[j] + f[j]);
        }
        f[i] = best;
    }
    f[0]
}

fn main() {
    println!("{}", max_score(vec![1, 5, 8]));
}

#[cfg(test)]
mod tests {
    use super::max_score;

    #[test]
    fn example1() {
        assert_eq!(max_score(vec![1, 5, 8]), 16);
    }

    #[test]
    fn example2() {
        assert_eq!(max_score(vec![4, 5, 2, 8, 9, 1, 3]), 42);
    }
}
