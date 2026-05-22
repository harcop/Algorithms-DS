/// LeetCode #1238 - Circular Permutation in Binary Representation
fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
    let len = 1usize << n;
    (0..len)
        .map(|i| (i as i32) ^ ((i as i32) >> 1) ^ start)
        .collect()
}

fn main() {
    println!("{:?}", circular_permutation(2, 3));
}

#[cfg(test)]
mod tests {
    use super::circular_permutation;

    #[test]
    fn example_one() {
        assert_eq!(circular_permutation(2, 3), vec![3, 2, 0, 1]);
    }

    #[test]
    fn example_two() {
        assert_eq!(circular_permutation(3, 2), vec![2, 3, 1, 0, 4, 5, 7, 6]);
    }
}
