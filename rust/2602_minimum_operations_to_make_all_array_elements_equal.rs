/// LeetCode #2602 - Minimum Operations to Make All Array Elements Equal
fn min_operations(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i64> {
    nums.sort_unstable();
    let n = nums.len();
    let mut s = vec![0i64; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + nums[i] as i64;
    }

    let search = |x: i32| -> usize {
        let mut l = 0usize;
        let mut r = n;
        while l < r {
            let mid = (l + r) / 2;
            if nums[mid] >= x {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l
    };

    let mut ans = Vec::with_capacity(queries.len());
    for x in queries {
        let i = search(x + 1);
        let mut t = s[n] - s[i] - (n - i) as i64 * x as i64;
        let j = search(x);
        t += x as i64 * j as i64 - s[j];
        ans.push(t);
    }
    ans
}

fn main() {
    println!("{:?}", min_operations(vec![3, 1, 6, 8], vec![1, 5]));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example_one() {
        assert_eq!(
            min_operations(vec![3, 1, 6, 8], vec![1, 5]),
            vec![14, 10]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(min_operations(vec![2, 9, 6, 3], vec![10]), vec![20]);
    }
}
