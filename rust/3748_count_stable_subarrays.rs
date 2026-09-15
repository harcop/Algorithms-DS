/// LeetCode #3748 - Count Stable Subarrays
fn count_stable_subarrays(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
    let n = nums.len();
    let mut seg = Vec::new();
    let mut s = vec![0i64];
    let mut l = 0usize;
    for r in 0..n {
        if r == n - 1 || nums[r] > nums[r + 1] {
            seg.push(l);
            let k = (r - l + 1) as i64;
            s.push(s.last().unwrap() + k * (k + 1) / 2);
            l = r + 1;
        }
    }
    let mut ans = Vec::with_capacity(queries.len());
    for q in queries {
        let left = q[0] as usize;
        let right = q[1] as usize;
        let i = seg.partition_point(|&x| x <= left);
        let j = seg.partition_point(|&x| x <= right) as isize - 1;
        if i as isize > j {
            let k = (right - left + 1) as i64;
            ans.push(k * (k + 1) / 2);
        } else {
            let j = j as usize;
            let a = (seg[i] - left) as i64;
            let b = (right - seg[j] + 1) as i64;
            ans.push(a * (a + 1) / 2 + s[j] - s[i] + b * (b + 1) / 2);
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        count_stable_subarrays(vec![3, 1, 2], vec![vec![0, 1], vec![1, 2], vec![0, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::count_stable_subarrays;

    #[test]
    fn example1() {
        assert_eq!(
            count_stable_subarrays(vec![3, 1, 2], vec![vec![0, 1], vec![1, 2], vec![0, 2]]),
            vec![2, 3, 4]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            count_stable_subarrays(vec![2, 2], vec![vec![0, 1], vec![0, 0]]),
            vec![3, 1]
        );
    }
}
