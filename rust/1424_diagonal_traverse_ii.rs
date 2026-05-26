/// LeetCode #1424 - Diagonal Traverse Ii
fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let mut buckets: Vec<Vec<i32>> = Vec::new();
    for (r, row) in nums.iter().enumerate() {
        for (c, &val) in row.iter().enumerate() {
            let d = r + c;
            if d >= buckets.len() {
                buckets.resize(d + 1, Vec::new());
            }
            buckets[d].push(val);
        }
    }
    let mut ans = Vec::new();
    for mut bucket in buckets {
        bucket.reverse();
        ans.extend(bucket);
    }
    ans
}

fn main() {
    println!("{:?}", find_diagonal_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]));
}

#[cfg(test)]
mod tests {
    use super::find_diagonal_order;

    #[test]
    fn example_one() {
        assert_eq!(
            find_diagonal_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 4, 2, 7, 5, 3, 8, 6, 9]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(find_diagonal_order(vec![vec![1, 2, 3, 4, 5]]), vec![1, 2, 3, 4, 5]);
    }
}
