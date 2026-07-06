/// LeetCode #2251 - Number of Flowers in Full Bloom
fn full_bloom_flowers(flowers: Vec<Vec<i32>>, persons: Vec<i32>) -> Vec<i32> {
    let mut starts: Vec<i32> = flowers.iter().map(|f| f[0]).collect();
    let mut ends: Vec<i32> = flowers.iter().map(|f| f[1]).collect();
    starts.sort_unstable();
    ends.sort_unstable();

    persons
        .iter()
        .map(|&person| {
            let started = upper_bound(&starts, person) as i32;
            let ended = lower_bound(&ends, person) as i32;
            started - ended
        })
        .collect()
}

fn upper_bound(arr: &[i32], target: i32) -> usize {
    let mut lo = 0usize;
    let mut hi = arr.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if arr[mid] <= target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

fn lower_bound(arr: &[i32], target: i32) -> usize {
    let mut lo = 0usize;
    let mut hi = arr.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if arr[mid] < target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

fn main() {
    println!(
        "{:?}",
        full_bloom_flowers(vec![vec![1, 6], vec![3, 7], vec![9, 12]], vec![2, 3, 7, 11])
    );
}

#[cfg(test)]
mod tests {
    use super::full_bloom_flowers;

    #[test]
    fn example_one() {
        assert_eq!(
            full_bloom_flowers(vec![vec![1, 6], vec![3, 7], vec![9, 12]], vec![2, 3, 7, 11]),
            vec![1, 2, 1, 1]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            full_bloom_flowers(vec![vec![1, 10], vec![3, 3]], vec![3, 3, 3]),
            vec![2, 2, 2]
        );
    }
}
