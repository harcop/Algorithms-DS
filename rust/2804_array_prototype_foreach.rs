/// LeetCode #2804 - Array Prototype ForEach (JS problem; Rust analogue)
fn for_each_mut<F>(arr: &mut Vec<i32>, mut callback: F)
where
    F: FnMut(usize, &mut Vec<i32>),
{
    let n = arr.len();
    for i in 0..n {
        callback(i, arr);
    }
}

fn main() {
    let mut arr = vec![1, 2, 3];
    for_each_mut(&mut arr, |i, a| a[i] *= 2);
    println!("{:?}", arr);
}

#[cfg(test)]
mod tests {
    use super::for_each_mut;

    #[test]
    fn example_one() {
        let mut arr = vec![1, 2, 3];
        for_each_mut(&mut arr, |i, a| a[i] *= 2);
        assert_eq!(arr, vec![2, 4, 6]);
    }

    #[test]
    fn example_three() {
        let mut arr = vec![1, 1, 0, 0];
        for_each_mut(&mut arr, |i, a| a[i] = 1 - a[i]);
        assert_eq!(arr, vec![0, 0, 1, 1]);
    }
}
