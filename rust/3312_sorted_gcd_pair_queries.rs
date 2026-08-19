/// LeetCode #3312 - Sorted GCD Pair Queries
fn gcd_values(nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
    let mx = *nums.iter().max().unwrap() as usize;
    let mut cnt = vec![0i64; mx + 1];
    for x in nums {
        cnt[x as usize] += 1;
    }
    let mut cnt_g = vec![0i64; mx + 1];
    for i in (1..=mx).rev() {
        let mut v = 0i64;
        let mut j = i;
        while j <= mx {
            v += cnt[j];
            cnt_g[i] -= cnt_g[j];
            j += i;
        }
        cnt_g[i] += v * (v - 1) / 2;
    }
    let mut s = vec![0i64; mx + 1];
    for i in 1..=mx {
        s[i] = s[i - 1] + cnt_g[i];
    }
    queries
        .into_iter()
        .map(|q| s.partition_point(|&x| x <= q) as i32)
        .collect()
}

fn main() {
    println!("{:?}", gcd_values(vec![2, 3, 4], vec![0, 2, 2]));
}

#[cfg(test)]
mod tests {
    use super::gcd_values;

    #[test]
    fn example1() {
        assert_eq!(gcd_values(vec![2, 3, 4], vec![0, 2, 2]), vec![1, 2, 2]);
    }

    #[test]
    fn example2() {
        assert_eq!(
            gcd_values(vec![4, 4, 2, 1], vec![5, 3, 1, 0]),
            vec![4, 2, 1, 1]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(gcd_values(vec![2, 2], vec![0, 0]), vec![2, 2]);
    }
}
