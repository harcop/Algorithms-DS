/// LeetCode #2948 - Make Lexicographically Smallest Array by Swapping Elements
fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
    let n = nums.len();
    let mut arr: Vec<(i32, usize)> = nums.iter().copied().zip(0..n).collect();
    arr.sort_unstable();
    let mut ans = vec![0; n];
    let mut i = 0;
    while i < n {
        let mut j = i + 1;
        while j < n && arr[j].0 - arr[j - 1].0 <= limit {
            j += 1;
        }
        let mut idx: Vec<usize> = arr[i..j].iter().map(|&(_, k)| k).collect();
        idx.sort_unstable();
        for (pos, &(x, _)) in idx.into_iter().zip(arr[i..j].iter()) {
            ans[pos] = x;
        }
        i = j;
    }
    ans
}

fn main() {
    println!("{:?}", lexicographically_smallest_array(vec![1, 5, 3, 9, 8], 2));
}

#[cfg(test)]
mod tests {
    use super::lexicographically_smallest_array;

    #[test]
    fn example_one() {
        assert_eq!(
            lexicographically_smallest_array(vec![1, 5, 3, 9, 8], 2),
            vec![1, 3, 5, 8, 9]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            lexicographically_smallest_array(vec![1, 7, 6, 18, 2, 1], 3),
            vec![1, 6, 7, 18, 1, 2]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            lexicographically_smallest_array(vec![1, 7, 28, 19, 10], 3),
            vec![1, 7, 28, 19, 10]
        );
    }
}
