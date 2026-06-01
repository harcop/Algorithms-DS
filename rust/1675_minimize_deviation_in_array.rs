/// LeetCode #1675 - Minimize Deviation In Array
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn minimum_deviation(nums: Vec<i32>) -> i32 {
    let mut a = nums;
    let mut heap = BinaryHeap::new();
    for (i, &x) in a.iter().enumerate() {
        let v = if x % 2 == 1 { x * 2 } else { x };
        a[i] = v;
        heap.push((v, i));
    }
    let mut mn = *a.iter().min().unwrap();
    let mut ans = heap.peek().unwrap().0 - mn;
    while let Some((mx, i)) = heap.pop() {
        ans = ans.min(mx - mn);
        if mx % 2 != 0 {
            break;
        }
        let h = mx / 2;
        a[i] = h;
        heap.push((h, i));
        mn = mn.min(h);
    }
    ans
}

fn main() {
    println!("{}", minimum_deviation(vec![1, 2, 8]));
}

#[cfg(test)]
mod tests {
    use super::minimum_deviation;

    #[test]
    fn example_one() {
        assert_eq!(minimum_deviation(vec![1, 2, 8]), 3);
    }
}
