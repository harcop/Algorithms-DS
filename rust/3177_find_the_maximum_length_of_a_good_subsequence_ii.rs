/// LeetCode #3177 - Find the Maximum Length of a Good Subsequence II
use std::collections::HashMap;

fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;
    let mut f = vec![vec![0; k + 1]; n];
    let mut mp: Vec<HashMap<i32, i32>> = vec![HashMap::new(); k + 1];
    let mut g = vec![[0i32; 3]; k + 1];
    let mut ans = 0;
    for i in 0..n {
        let x = nums[i];
        for h in 0..=k {
            f[i][h] = *mp[h].get(&x).unwrap_or(&0);
            if h > 0 {
                if g[h - 1][0] != x {
                    f[i][h] = f[i][h].max(g[h - 1][1]);
                } else {
                    f[i][h] = f[i][h].max(g[h - 1][2]);
                }
            }
            f[i][h] += 1;
            let e = mp[h].entry(x).or_insert(0);
            *e = (*e).max(f[i][h]);
            if g[h][0] != x {
                if f[i][h] >= g[h][1] {
                    g[h][2] = g[h][1];
                    g[h][1] = f[i][h];
                    g[h][0] = x;
                } else {
                    g[h][2] = g[h][2].max(f[i][h]);
                }
            } else {
                g[h][1] = g[h][1].max(f[i][h]);
            }
            ans = ans.max(f[i][h]);
        }
    }
    ans
}

fn main() {
    println!("{}", maximum_length(vec![1, 2, 1, 1, 3], 2));
}

#[cfg(test)]
mod tests {
    use super::maximum_length;

    #[test]
    fn example1() {
        assert_eq!(maximum_length(vec![1, 2, 1, 1, 3], 2), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_length(vec![1, 2, 3, 4, 5, 1], 0), 2);
    }
}
