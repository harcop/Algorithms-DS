/// LeetCode #3506 - Find Time Required to Eliminate Bacterial Strains
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn min_elimination_time(time_req: Vec<i32>, split_time: i32) -> i64 {
    let mut heap: BinaryHeap<Reverse<i64>> = time_req.into_iter().map(|x| Reverse(x as i64)).collect();
    let split_time = split_time as i64;
    while heap.len() > 1 {
        heap.pop();
        let Reverse(b) = heap.pop().unwrap();
        heap.push(Reverse(b + split_time));
    }
    heap.pop().unwrap().0
}

fn main() {
    println!("{}", min_elimination_time(vec![10, 4, 5], 2));
}

#[cfg(test)]
mod tests {
    use super::min_elimination_time;

    #[test]
    fn example1() {
        assert_eq!(min_elimination_time(vec![10, 4, 5], 2), 12);
    }

    #[test]
    fn example2() {
        assert_eq!(min_elimination_time(vec![10, 4], 5), 15);
    }
}
