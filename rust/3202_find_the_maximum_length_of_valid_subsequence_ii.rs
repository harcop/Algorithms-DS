/// LeetCode #3202 - Find the Maximum Length of Valid Subsequence II
fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut f = vec![vec![0i32; k]; k];
    let mut ans = 0;
    for x in nums {
        let x = (x % k as i32) as usize;
        for j in 0..k {
            let y = (j + k - x) % k;
            f[x][y] = f[y][x] + 1;
            ans = ans.max(f[x][y]);
        }
    }
    ans
}

fn main() {
    println!("{}", maximum_length(vec![1, 2, 3, 4, 5], 2));
}

#[cfg(test)]
mod tests {
    use super::maximum_length;

    #[test]
    fn example1() {
        assert_eq!(maximum_length(vec![1, 2, 3, 4, 5], 2), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_length(vec![1, 4, 2, 3, 1, 4], 3), 4);
    }
}
