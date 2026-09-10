/// LeetCode #3672 - Sum of Weighted Modes in Subarrays (premium)
use std::collections::{BinaryHeap, HashMap};

fn mode_weight(nums: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    // max-heap by (freq, -val) so higher freq, then smaller val
    let mut pq: BinaryHeap<(i32, i32)> = BinaryHeap::new();
    for &x in &nums[..k] {
        let e = cnt.entry(x).or_insert(0);
        *e += 1;
        pq.push((*e, -x));
    }
    let get_mode = |cnt: &HashMap<i32, i32>, pq: &mut BinaryHeap<(i32, i32)>| -> i64 {
        loop {
            let &(freq, neg) = pq.peek().unwrap();
            let val = -neg;
            if *cnt.get(&val).unwrap_or(&0) == freq {
                return freq as i64 * val as i64;
            }
            pq.pop();
        }
    };
    let mut ans = get_mode(&cnt, &mut pq);
    for i in k..nums.len() {
        let x = nums[i];
        let y = nums[i - k];
        *cnt.entry(x).or_insert(0) += 1;
        pq.push((cnt[&x], -x));
        *cnt.entry(y).or_insert(0) -= 1;
        pq.push((cnt[&y], -y));
        ans += get_mode(&cnt, &mut pq);
    }
    ans
}

fn main() {
    println!("{}", mode_weight(vec![1, 2, 2, 3], 3));
}

#[cfg(test)]
mod tests {
    use super::mode_weight;

    #[test]
    fn example1() {
        assert_eq!(mode_weight(vec![1, 2, 2, 3], 3), 8);
    }

    #[test]
    fn example2() {
        assert_eq!(mode_weight(vec![1, 2, 1, 2], 2), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(mode_weight(vec![4, 3, 4, 3], 3), 14);
    }
}
