/// LeetCode #2747 - Count Zero Request Servers
use std::collections::HashMap;

fn count_servers(n: i32, mut logs: Vec<Vec<i32>>, x: i32, queries: Vec<i32>) -> Vec<i32> {
    logs.sort_by_key(|e| e[1]);
    let m = queries.len();
    let mut qs: Vec<(i32, usize)> = queries.into_iter().enumerate().map(|(i, q)| (q, i)).collect();
    qs.sort_unstable();
    let mut ans = vec![0; m];
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let mut j = 0;
    let mut k = 0;
    for (r, i) in qs {
        let l = r - x;
        while k < logs.len() && logs[k][1] <= r {
            *cnt.entry(logs[k][0]).or_insert(0) += 1;
            k += 1;
        }
        while j < logs.len() && logs[j][1] < l {
            let id = logs[j][0];
            if let Some(v) = cnt.get_mut(&id) {
                *v -= 1;
                if *v == 0 {
                    cnt.remove(&id);
                }
            }
            j += 1;
        }
        ans[i] = n - cnt.len() as i32;
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        count_servers(
            3,
            vec![vec![1, 3], vec![2, 6], vec![1, 5]],
            5,
            vec![10, 11]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::count_servers;

    #[test]
    fn example_one() {
        assert_eq!(
            count_servers(
                3,
                vec![vec![1, 3], vec![2, 6], vec![1, 5]],
                5,
                vec![10, 11]
            ),
            vec![1, 2]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            count_servers(
                3,
                vec![vec![2, 4], vec![2, 1], vec![1, 2], vec![3, 1]],
                2,
                vec![3, 4]
            ),
            vec![0, 1]
        );
    }
}
