/// LeetCode #624 - Maximum Distance in Arrays
fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
    let mut min_v = arrays[0][0];
    let mut max_v = *arrays[0].last().unwrap();
    let mut ans = 0;
    for arr in arrays.iter().skip(1) {
        let a0 = arr[0];
        let a1 = *arr.last().unwrap();
        ans = ans.max((a1 - min_v).abs()).max((max_v - a0).abs());
        min_v = min_v.min(a0);
        max_v = max_v.max(a1);
    }
    ans
}

fn main() {
    let arrays = vec![vec![1, 2, 3], vec![4, 5], vec![1, 2, 3]];
    println!("{}", max_distance(arrays));
}

#[cfg(test)]
mod tests {
    use super::max_distance;

    #[test]
    fn example_one() {
        let arrays = vec![vec![1, 2, 3], vec![4, 5], vec![1, 2, 3]];
        assert_eq!(max_distance(arrays), 4);
    }

    #[test]
    fn example_two() {
        let arrays = vec![vec![1], vec![1]];
        assert_eq!(max_distance(arrays), 0);
    }
}
