/// LeetCode #1054 - Distant Barcodes
use std::collections::BinaryHeap;

fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for b in barcodes {
        *cnt.entry(b).or_default() += 1;
    }
    let mut heap: BinaryHeap<(i32, i32)> = cnt.into_iter().map(|(k, v)| (v, k)).collect();
    let mut ans = Vec::new();
    let mut prev: Option<(i32, i32)> = None;
    while let Some((mut c, val)) = heap.pop() {
        ans.push(val);
        c -= 1;
        if let Some((pc, pv)) = prev.take() {
            if pc > 0 {
                heap.push((pc, pv));
            }
        }
        if c > 0 {
            prev = Some((c, val));
        }
    }
    ans
}

fn main() {
    println!("{:?}", rearrange_barcodes(vec![1, 1, 1, 2, 2, 2]));
}

#[cfg(test)]
mod tests {
    use super::rearrange_barcodes;

    #[test]
    fn example_one() {
        let out = rearrange_barcodes(vec![1, 1, 1, 2, 2, 2]);
        assert_eq!(out.len(), 6);
        for w in out.windows(2) {
            assert_ne!(w[0], w[1]);
        }
    }
}
