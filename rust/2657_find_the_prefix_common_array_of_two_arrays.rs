/// LeetCode #2657 - Find the Prefix Common Array of Two Arrays
fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let n = a.len();
    let mut ans = Vec::with_capacity(n);
    let mut x: u64 = 0;
    let mut y: u64 = 0;
    for i in 0..n {
        x |= 1u64 << a[i];
        y |= 1u64 << b[i];
        ans.push((x & y).count_ones() as i32);
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        find_the_prefix_common_array(vec![1, 3, 2, 4], vec![3, 1, 2, 4])
    );
}

#[cfg(test)]
mod tests {
    use super::find_the_prefix_common_array;

    #[test]
    fn example_one() {
        assert_eq!(
            find_the_prefix_common_array(vec![1, 3, 2, 4], vec![3, 1, 2, 4]),
            vec![0, 2, 3, 4]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_the_prefix_common_array(vec![2, 3, 1], vec![3, 1, 2]),
            vec![0, 1, 3]
        );
    }
}
