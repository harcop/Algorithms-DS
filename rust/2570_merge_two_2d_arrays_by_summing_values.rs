/// LeetCode #2570 - Merge Two 2D Arrays by Summing Values
fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut cnt = [0i32; 1001];
    for x in nums1.iter().chain(nums2.iter()) {
        cnt[x[0] as usize] += x[1];
    }
    let mut ans = Vec::new();
    for i in 1..=1000 {
        if cnt[i] > 0 {
            ans.push(vec![i as i32, cnt[i]]);
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        merge_arrays(
            vec![vec![1, 2], vec![2, 3], vec![4, 5]],
            vec![vec![1, 4], vec![3, 2], vec![4, 1]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::merge_arrays;

    #[test]
    fn example_one() {
        assert_eq!(
            merge_arrays(
                vec![vec![1, 2], vec![2, 3], vec![4, 5]],
                vec![vec![1, 4], vec![3, 2], vec![4, 1]]
            ),
            vec![vec![1, 6], vec![2, 3], vec![3, 2], vec![4, 6]]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            merge_arrays(
                vec![vec![2, 4], vec![3, 6], vec![5, 5]],
                vec![vec![1, 3], vec![4, 3]]
            ),
            vec![
                vec![1, 3],
                vec![2, 4],
                vec![3, 6],
                vec![4, 3],
                vec![5, 5]
            ]
        );
    }
}
