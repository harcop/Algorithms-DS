/// LeetCode #1642 - Furthest Building You Can Reach
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
    let mut heap = BinaryHeap::new();
    let mut b = bricks as i64;
    let mut lad = ladders;
    let mut pos = 0usize;
    for i in 0..heights.len().saturating_sub(1) {
        let diff = heights[i + 1] - heights[i];
        if diff <= 0 { pos = i + 1; continue; }
        heap.push(Reverse(diff));
        b -= diff as i64;
        if b < 0 {
            if lad == 0 { break; }
            lad -= 1;
            if let Some(Reverse(x)) = heap.pop() { b += x as i64; }
        }
        pos = i + 1;
    }
    pos as i32
}
fn main() { println!("{}", furthest_building(vec![4,2,7,6,9,14,12], 5, 1)); }
#[cfg(test)]
mod tests {
    use super::furthest_building;
    #[test]
    fn example_one() { assert_eq!(furthest_building(vec![4,2,7,6,9,14,12], 5, 1), 4); }
}