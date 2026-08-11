/// LeetCode #3152 - Special Array II
fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let n = nums.len();
    let mut d: Vec<usize> = (0..n).collect();
    for i in 1..n {
        if nums[i] % 2 != nums[i - 1] % 2 {
            d[i] = d[i - 1];
        }
    }
    queries
        .into_iter()
        .map(|q| {
            let f = q[0] as usize;
            let t = q[1] as usize;
            d[t] <= f
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        is_array_special(vec![3, 4, 1, 2, 6], vec![vec![0, 4]])
    );
}

#[cfg(test)]
mod tests {
    use super::is_array_special;

    #[test]
    fn example1() {
        assert_eq!(
            is_array_special(vec![3, 4, 1, 2, 6], vec![vec![0, 4]]),
            vec![false]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            is_array_special(vec![4, 3, 1, 6], vec![vec![0, 2], vec![2, 3]]),
            vec![false, true]
        );
    }
}
