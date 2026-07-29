/// LeetCode #2757 - Generate Circular Array Values (JS problem; Rust analogue)
/// Returns a value from a circular array given a current index and a jump amount.
/// A positive jump moves forward; a negative jump moves backward.
fn circular_array_generator(arr: &[i32], start: usize) -> impl Iterator<Item = i32> + '_ {
    let n = arr.len();
    std::iter::successors(Some(start), move |&i| Some((i + 1) % n))
        .map(move |i| arr[i])
}

fn get_value(arr: &[i32], index: i32, jump: i32) -> i32 {
    let n = arr.len() as i32;
    let next = ((index + jump) % n + n) % n;
    arr[next as usize]
}

fn main() {
    let arr = vec![1, 2, 3, 4, 5];
    println!("{}", get_value(&arr, 0, 1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_forward() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(get_value(&arr, 4, 1), 1);
    }

    #[test]
    fn example_backward() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(get_value(&arr, 0, -1), 5);
    }

    #[test]
    fn example_circular_iterator() {
        let arr = vec![10, 20, 30];
        let vals: Vec<i32> = circular_array_generator(&arr, 1).take(5).collect();
        assert_eq!(vals, vec![20, 30, 10, 20, 30]);
    }
}
