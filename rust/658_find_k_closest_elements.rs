/// LeetCode #658 - Find K Closest Elements
fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let k = k as usize;
    let mut lo = 0usize;
    let mut hi = arr.len() - k;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if x - arr[mid] > arr[mid + k] - x {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    arr[lo..lo + k].to_vec()
}

fn main() {
    println!("{:?}", find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3));
}

#[cfg(test)]
mod tests {
    use super::find_closest_elements;

    #[test]
    fn example_one() {
        assert_eq!(find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3), vec![1, 2, 3, 4]);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_closest_elements(vec![1, 2, 3, 4, 5], 4, -1), vec![1, 2, 3, 4]);
    }
}
