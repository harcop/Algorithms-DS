/// LeetCode #2624 - Snail Traversal (JS problem; Rust function analogue)
fn snail(nums: &[i32], rows_count: usize, cols_count: usize) -> Vec<Vec<i32>> {
    if rows_count * cols_count != nums.len() {
        return vec![];
    }
    let mut ans = vec![vec![0; cols_count]; rows_count];
    let mut i: isize = 0;
    let mut j: usize = 0;
    let mut k: isize = 1;
    for &v in nums {
        ans[i as usize][j] = v;
        i += k;
        if i == rows_count as isize || i == -1 {
            i -= k;
            k = -k;
            j += 1;
        }
    }
    ans
}

fn main() {
    println!("{:?}", snail(&[1, 2, 3, 4], 1, 4));
}

#[cfg(test)]
mod tests {
    use super::snail;

    #[test]
    fn example_one() {
        let nums = [
            19, 10, 3, 7, 9, 8, 5, 2, 1, 17, 16, 14, 12, 18, 6, 13, 11, 20, 4, 15,
        ];
        assert_eq!(
            snail(&nums, 5, 4),
            vec![
                vec![19, 17, 16, 15],
                vec![10, 1, 14, 4],
                vec![3, 2, 12, 20],
                vec![7, 5, 18, 11],
                vec![9, 8, 6, 13],
            ]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(snail(&[1, 2, 3, 4], 1, 4), vec![vec![1, 2, 3, 4]]);
    }

    #[test]
    fn example_three() {
        assert_eq!(snail(&[1, 3], 2, 2), Vec::<Vec<i32>>::new());
    }
}
