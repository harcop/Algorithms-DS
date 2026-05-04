/// LeetCode #275 - H-Index II (citations sorted ascending)
fn h_index(citations: Vec<i32>) -> i32 {
    let n = citations.len();
    let mut lo = 0usize;
    let mut hi = n;
    while lo < hi {
        let mid = lo + (hi - lo + 1) / 2;
        if citations[n - mid] >= mid as i32 {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo as i32
}

fn main() {
    println!("{}", h_index(vec![0, 1, 3, 5, 6]));
}

#[cfg(test)]
mod tests {
    use super::h_index;

    #[test]
    fn example_one() {
        assert_eq!(h_index(vec![0, 1, 3, 5, 6]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(h_index(vec![1, 2, 100]), 2);
    }
}
