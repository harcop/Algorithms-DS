/// LeetCode #3975 - Filter Occupied Intervals
fn filter_occupied_intervals(
    mut occupied: Vec<Vec<i32>>,
    free_start: i32,
    free_end: i32,
) -> Vec<Vec<i32>> {
    occupied.sort_unstable_by_key(|x| x[0]);
    let mut busy = vec![occupied[0].clone()];
    for interval in occupied.into_iter().skip(1) {
        if busy.last().unwrap()[1] + 1 < interval[0] {
            busy.push(interval);
        } else {
            let last = busy.last_mut().unwrap();
            last[1] = last[1].max(interval[1]);
        }
    }
    let mut ans = Vec::new();
    for interval in busy {
        if interval[1] < free_start || free_end < interval[0] {
            ans.push(interval);
        } else {
            if interval[0] < free_start {
                ans.push(vec![interval[0], free_start - 1]);
            }
            if interval[1] > free_end {
                ans.push(vec![free_end + 1, interval[1]]);
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        filter_occupied_intervals(vec![vec![2, 6], vec![4, 8], vec![10, 10], vec![10, 12], vec![14, 16]], 7, 11)
    );
}

#[cfg(test)]
mod tests {
    use super::filter_occupied_intervals;

    #[test]
    fn example1() {
        assert_eq!(
            filter_occupied_intervals(
                vec![vec![2, 6], vec![4, 8], vec![10, 10], vec![10, 12], vec![14, 16]],
                7,
                11
            ),
            vec![vec![2, 6], vec![12, 12], vec![14, 16]]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            filter_occupied_intervals(vec![vec![1, 5], vec![2, 3]], 3, 8),
            vec![vec![1, 2]]
        );
    }
}
