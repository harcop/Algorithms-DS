/// LeetCode #1343 - Number Of Sub Arrays Of Size K And Average Greater Than Or Equal To Threshold

fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
    let k = k as usize;
    let need = threshold as i64 * k as i64;
    let mut sum: i64 = 0;
    let mut count = 0i32;
    for i in 0..arr.len() {
        sum += arr[i] as i64;
        if i >= k {
            sum -= arr[i - k] as i64;
        }
        if i + 1 >= k && sum >= need {
            count += 1;
        }
    }
    count
}

fn main() {
    println!("{}", num_of_subarrays(vec![2, 2, 2, 2, 5, 5, 5, 8], 3, 4));
}

#[cfg(test)]
mod tests {
    use super::num_of_subarrays;

    #[test]
    fn example_one() {
        assert_eq!(num_of_subarrays(vec![2, 2, 2, 2, 5, 5, 5, 8], 3, 4), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_of_subarrays(vec![1, 1, 1, 1, 1], 1, 0), 5);
    }
}
