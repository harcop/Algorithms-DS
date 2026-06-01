/// LeetCode #1705 - Maximum Number Of Eaten Apples
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
    let mut heap = BinaryHeap::new();
    let mut ans = 0i32;
    let n = apples.len();
    for day in 0..200000 {
        if day < n && apples[day] > 0 {
            heap.push(Reverse((days[day] + day as i32, apples[day])));
        }
        while heap.peek().map(|Reverse((exp, _))| *exp <= day as i32).unwrap_or(false) {
            heap.pop();
        }
        if let Some(Reverse((_, mut cnt))) = heap.pop() {
            ans += 1;
            cnt -= 1;
            if cnt > 0 { heap.push(Reverse((days.get(day).copied().unwrap_or(0) + day as i32, cnt))); }
        } else if day >= n { break; }
    }
    ans
}
fn main() { println!("{}", eaten_apples(vec![1,2,3,5,2], vec![3,1,1,4,2])); }
#[cfg(test)]
mod tests {
    use super::eaten_apples;
    #[test]
    fn example_one() { assert_eq!(eaten_apples(vec![1,2,3,5,2], vec![3,1,1,4,2]), 7); }
}