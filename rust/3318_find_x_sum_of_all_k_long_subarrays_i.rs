/// LeetCode #3318 - Find X-Sum of All K-Long Subarrays I
fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let k = k as usize;
    let x = x as usize;
    let n = nums.len();
    let mut ans = Vec::with_capacity(n - k + 1);
    for i in 0..=n - k {
        let mut cnt = [0i32; 51];
        for &v in &nums[i..i + k] {
            cnt[v as usize] += 1;
        }
        let mut items: Vec<(i32, i32)> = (1..=50)
            .filter(|&v| cnt[v] > 0)
            .map(|v| (cnt[v], v as i32))
            .collect();
        items.sort_by(|a, b| b.cmp(a));
        let s: i32 = items
            .iter()
            .take(x)
            .map(|&(c, v)| c * v)
            .sum();
        ans.push(s);
    }
    ans
}

fn main() {
    println!("{:?}", find_x_sum(vec![1, 1, 2, 2, 3, 4, 2, 3], 6, 2));
}

#[cfg(test)]
mod tests {
    use super::find_x_sum;

    #[test]
    fn example1() {
        assert_eq!(
            find_x_sum(vec![1, 1, 2, 2, 3, 4, 2, 3], 6, 2),
            vec![6, 10, 12]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            find_x_sum(vec![3, 8, 7, 8, 7, 5], 2, 2),
            vec![11, 15, 15, 15, 12]
        );
    }
}
