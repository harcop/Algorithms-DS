/// LeetCode #2677 - Chunk Array (JS problem; Rust slice analogue)
fn chunk(arr: Vec<i32>, size: usize) -> Vec<Vec<i32>> {
    let mut ans = Vec::new();
    let mut i = 0;
    let n = arr.len();
    while i < n {
        ans.push(arr[i..(i + size).min(n)].to_vec());
        i += size;
    }
    ans
}

fn main() {
    println!("{:?}", chunk(vec![1, 2, 3, 4, 5], 1));
}

#[cfg(test)]
mod tests {
    use super::chunk;

    #[test]
    fn example_one() {
        assert_eq!(
            chunk(vec![1, 2, 3, 4, 5], 1),
            vec![vec![1], vec![2], vec![3], vec![4], vec![5]]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            chunk(vec![1, 9, 6, 3, 2], 3),
            vec![vec![1, 9, 6], vec![3, 2]]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(chunk(vec![8, 5, 3, 2, 6], 6), vec![vec![8, 5, 3, 2, 6]]);
    }

    #[test]
    fn example_four() {
        assert!(chunk(vec![], 1).is_empty());
    }
}
