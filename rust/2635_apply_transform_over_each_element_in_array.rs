/// LeetCode #2635 - Apply Transform Over Each Element in Array (JS problem; Rust map analogue)
fn map(mut arr: Vec<i32>, fn_: impl Fn(i32, usize) -> i32) -> Vec<i32> {
    for i in 0..arr.len() {
        arr[i] = fn_(arr[i], i);
    }
    arr
}

fn main() {
    println!("{:?}", map(vec![1, 2, 3], |n, _| n + 1));
}

#[cfg(test)]
mod tests {
    use super::map;

    #[test]
    fn example_one() {
        assert_eq!(map(vec![1, 2, 3], |n, _| n + 1), vec![2, 3, 4]);
    }

    #[test]
    fn example_two() {
        assert_eq!(map(vec![1, 2, 3], |n, i| n + i as i32), vec![1, 3, 5]);
    }

    #[test]
    fn example_three() {
        assert_eq!(map(vec![10, 20, 30], |_, _| 42), vec![42, 42, 42]);
    }
}
