/// LeetCode #3711 - Maximum Transactions Without Negative Balance
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn max_transactions(transactions: Vec<i32>) -> i32 {
    let mut heap = BinaryHeap::new(); // min-heap via Reverse
    let mut curr = 0i64;
    for x in transactions {
        heap.push(Reverse(x));
        curr += x as i64;
        if curr < 0 {
            if let Some(Reverse(v)) = heap.pop() {
                curr -= v as i64;
            }
        }
    }
    heap.len() as i32
}

fn main() {
    println!("{}", max_transactions(vec![2, -5, 3, -1, -2]));
}

#[cfg(test)]
mod tests {
    use super::max_transactions;

    #[test]
    fn example1() {
        assert_eq!(max_transactions(vec![2, -5, 3, -1, -2]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(max_transactions(vec![-1, -2, -3]), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(max_transactions(vec![3, -2, 3, -2, 1, -1]), 6);
    }
}
