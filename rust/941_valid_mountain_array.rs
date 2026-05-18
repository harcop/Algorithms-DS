/// LeetCode #941 - Valid Mountain Array

fn valid_mountain_array(arr: Vec<i32>) -> bool {
    let n = arr.len();
    if n < 3 {
        return false;
    }
    let mut i = 0;
    while i + 1 < n && arr[i] < arr[i + 1] {
        i += 1;
    }
    if i == 0 || i == n - 1 {
        return false;
    }
    while i + 1 < n && arr[i] > arr[i + 1] {
        i += 1;
    }
    i == n - 1
}

fn main() {
    println!("{}", valid_mountain_array(vec![0, 2, 3, 4, 5, 2, 1, 0]));
}

#[cfg(test)]
mod tests {
    use super::valid_mountain_array;

    #[test]
    fn example_one() {
        assert!(valid_mountain_array(vec![0, 2, 3, 4, 5, 2, 1, 0]));
    }

    #[test]
    fn example_two() {
        assert!(!valid_mountain_array(vec![3, 5, 5]));
    }

    #[test]
    fn example_three() {
        assert!(!valid_mountain_array(vec![3, 5, 5]));
    }
}
