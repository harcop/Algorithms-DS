/// LeetCode #978 - Longest Turbulent Subarray
fn max_turbulence_size(arr: Vec<i32>) -> i32 {
    if arr.is_empty() {
        return 0;
    }
    let mut best = 1i32;
    let mut inc = 1i32;
    let mut dec = 1i32;
    for i in 1..arr.len() {
        if arr[i] > arr[i - 1] {
            inc = dec + 1;
            dec = 1;
        } else if arr[i] < arr[i - 1] {
            dec = inc + 1;
            inc = 1;
        } else {
            inc = 1;
            dec = 1;
        }
        best = best.max(inc).max(dec);
    }
    best
}

fn main() {
    println!("{}", max_turbulence_size(vec![9, 4, 2, 10, 7, 8, 8, 1, 9]));
}

#[cfg(test)]
mod tests {
    use super::max_turbulence_size;

    #[test]
    fn example_one() {
        assert_eq!(max_turbulence_size(vec![9, 4, 2, 10, 7, 8, 8, 1, 9]), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_turbulence_size(vec![4, 8, 12, 16]), 2);
    }
}
