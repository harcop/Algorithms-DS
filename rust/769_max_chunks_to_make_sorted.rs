/// LeetCode #769 - Max Chunks To Make Sorted
fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
    let mut mx = -1i32;
    let mut ans = 0i32;
    for (i, &x) in arr.iter().enumerate() {
        mx = mx.max(x);
        if mx == i as i32 {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", max_chunks_to_sorted(vec![4, 3, 2, 1, 0]));
}

#[cfg(test)]
mod tests {
    use super::max_chunks_to_sorted;

    #[test]
    fn example_one() {
        assert_eq!(max_chunks_to_sorted(vec![4, 3, 2, 1, 0]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_chunks_to_sorted(vec![1, 0, 2, 3, 4]), 4);
    }
}
