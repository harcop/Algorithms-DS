/// LeetCode #2951 - Find the Peaks
fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
    (1..mountain.len() - 1)
        .filter(|&i| mountain[i - 1] < mountain[i] && mountain[i] > mountain[i + 1])
        .map(|i| i as i32)
        .collect()
}

fn main() {
    println!("{:?}", find_peaks(vec![1, 4, 3, 8, 5]));
}

#[cfg(test)]
mod tests {
    use super::find_peaks;

    #[test]
    fn example_one() {
        assert_eq!(find_peaks(vec![2, 4, 4]), Vec::<i32>::new());
    }

    #[test]
    fn example_two() {
        assert_eq!(find_peaks(vec![1, 4, 3, 8, 5]), vec![1, 3]);
    }
}
