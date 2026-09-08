/// LeetCode #3636 - Threshold Majority Queries
use std::collections::HashMap;

fn subarray_majority(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut sorted_nums: Vec<i32> = nums.clone();
    sorted_nums.sort_unstable();
    sorted_nums.dedup();
    let num_to_idx: HashMap<i32, usize> = sorted_nums
        .iter()
        .enumerate()
        .map(|(i, &x)| (x, i))
        .collect();

    let m = num_to_idx.len();
    let mut cnt = vec![0i32; m];
    let mut cnt2 = vec![0i32; nums.len() + 1];
    let mut max_freq = 0i32;

    let mut add = |i: usize, cnt: &mut [i32], cnt2: &mut [i32], max_freq: &mut i32| {
        let idx = num_to_idx[&nums[i]];
        if cnt[idx] > 0 {
            cnt2[cnt[idx] as usize] -= 1;
        }
        cnt[idx] += 1;
        cnt2[cnt[idx] as usize] += 1;
        *max_freq = (*max_freq).max(cnt[idx]);
    };

    let mut remove = |i: usize, cnt: &mut [i32], cnt2: &mut [i32], max_freq: &mut i32| {
        let idx = num_to_idx[&nums[i]];
        cnt2[cnt[idx] as usize] -= 1;
        if cnt2[*max_freq as usize] == 0 {
            *max_freq -= 1;
        }
        cnt[idx] -= 1;
        if cnt[idx] > 0 {
            cnt2[cnt[idx] as usize] += 1;
        }
    };

    let get_ans = |t: i32, cnt: &[i32], max_freq: i32| -> i32 {
        if max_freq < t {
            return -1;
        }
        for i in 0..cnt.len() {
            if cnt[i] == max_freq {
                return sorted_nums[i];
            }
        }
        -1
    };

    let block_size = ((nums.len() as f64).sqrt() as usize).max(1);
    let mut idxs: Vec<usize> = (0..queries.len()).collect();
    idxs.sort_by(|&a, &b| {
        let ia = queries[a][0] as usize / block_size;
        let ib = queries[b][0] as usize / block_size;
        if ia != ib {
            ia.cmp(&ib)
        } else if ia & 1 == 1 {
            queries[a][1].cmp(&queries[b][1])
        } else {
            queries[b][1].cmp(&queries[a][1])
        }
    });

    let mut result = vec![-1; queries.len()];
    let mut left: i32 = 0;
    let mut right: i32 = -1;
    for &qi in &idxs {
        let l = queries[qi][0];
        let r = queries[qi][1];
        let t = queries[qi][2];
        while left > l {
            left -= 1;
            add(left as usize, &mut cnt, &mut cnt2, &mut max_freq);
        }
        while right < r {
            right += 1;
            add(right as usize, &mut cnt, &mut cnt2, &mut max_freq);
        }
        while left < l {
            remove(left as usize, &mut cnt, &mut cnt2, &mut max_freq);
            left += 1;
        }
        while right > r {
            remove(right as usize, &mut cnt, &mut cnt2, &mut max_freq);
            right -= 1;
        }
        result[qi] = get_ans(t, &cnt, max_freq);
    }
    result
}

fn main() {
    println!(
        "{:?}",
        subarray_majority(
            vec![1, 1, 2, 2, 1, 1],
            vec![vec![0, 5, 4], vec![0, 3, 3], vec![2, 3, 2]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::subarray_majority;

    #[test]
    fn example1() {
        assert_eq!(
            subarray_majority(
                vec![1, 1, 2, 2, 1, 1],
                vec![vec![0, 5, 4], vec![0, 3, 3], vec![2, 3, 2]]
            ),
            vec![1, -1, 2]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            subarray_majority(
                vec![3, 2, 3, 2, 3, 2, 3],
                vec![vec![0, 6, 4], vec![1, 5, 2], vec![2, 4, 1], vec![3, 3, 1]]
            ),
            vec![3, 2, 3, 2]
        );
    }
}
