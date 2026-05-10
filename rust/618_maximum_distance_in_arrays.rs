/// LeetCode #618 - Maximum Distance in Arrays
fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
    let mut min_v = arrays[0][0];
    let mut max_v = *arrays[0].last().unwrap();
    let mut ans = 0i32;
    for i in 1..arrays.len() {
        let a = &arrays[i];
        let cur_min = a[0];
        let cur_max = *a.last().unwrap();
        ans = ans.max((max_v - cur_min).abs()).max((cur_max - min_v).abs());
        min_v = min_v.min(cur_min);
        max_v = max_v.max(cur_max);
    }
    ans
}

fn main() {
    println!(
        "{}",
        max_distance(vec![vec![1, 2, 3], vec![4, 5], vec![1, 2, 3]])
    );
}

#[cfg(test)]
mod tests {
    use super::max_distance;

    #[test]
    fn example_one() {
        assert_eq!(
            max_distance(vec![vec![1, 2, 3], vec![4, 5], vec![1, 2, 3]]),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(max_distance(vec![vec![1], vec![1]]), 0);
    }
}
