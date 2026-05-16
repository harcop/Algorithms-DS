/// LeetCode #852 - Peak Index in a Mountain Array
fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
    let mut lo = 0;
    let mut hi = arr.len() - 1;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if arr[mid] < arr[mid + 1] {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo as i32
}

fn main() {
    println!("{}", peak_index_in_mountain_array(vec![0, 1, 0]));
}

#[cfg(test)]
mod tests {
    use super::peak_index_in_mountain_array;

    #[test]
    fn example_one() {
        assert_eq!(peak_index_in_mountain_array(vec![0, 1, 0]), 1);
    }
}
