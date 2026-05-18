/// LeetCode #975 - Odd Even Jump
fn odd_even_jumps(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut higher = vec![n; n];
    let mut lower = vec![n; n];
    let mut stack: Vec<usize> = Vec::new();
    for i in (0..n).rev() {
        while stack.last().is_some_and(|&j| arr[j] < arr[i]) {
            stack.pop();
        }
        if let Some(&j) = stack.last() {
            higher[i] = j;
        }
        stack.push(i);
    }
    stack.clear();
    for i in (0..n).rev() {
        while stack.last().is_some_and(|&j| arr[j] > arr[i]) {
            stack.pop();
        }
        if let Some(&j) = stack.last() {
            lower[i] = j;
        }
        stack.push(i);
    }
    let mut odd = vec![false; n];
    let mut even = vec![false; n];
    odd[n - 1] = true;
    even[n - 1] = true;
    for i in (0..n - 1).rev() {
        odd[i] = higher[i] < n && even[higher[i]];
        even[i] = lower[i] < n && odd[lower[i]];
    }
    odd.iter().filter(|&&b| b).count() as i32
}

fn main() {
    println!("{}", odd_even_jumps(vec![10, 13, 12, 14, 15]));
}

#[cfg(test)]
mod tests {
    use super::odd_even_jumps;

    #[test]
    fn example_one() {
        assert_eq!(odd_even_jumps(vec![10, 13, 12, 14, 15]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(odd_even_jumps(vec![2, 3, 1, 1, 4]), 3);
    }
}
