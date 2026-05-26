/// LeetCode #1405 - Longest Happy String
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
    #[derive(Eq, PartialEq)]
    struct Item(char, i32);
    impl Ord for Item {
        fn cmp(&self, other: &Self) -> Ordering {
            self.1.cmp(&other.1)
        }
    }
    impl PartialOrd for Item {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    let mut heap = BinaryHeap::new();
    for (ch, n) in [('a', a), ('b', b), ('c', c)] {
        if n > 0 {
            heap.push(Item(ch, n));
        }
    }
    let mut ans = String::new();
    while let Some(Item(ch, mut n)) = heap.pop() {
        let same = ans.len() >= 2
            && ans.as_bytes()[ans.len() - 1] == ch as u8
            && ans.as_bytes()[ans.len() - 2] == ch as u8;
        let use_cnt = if same { 0 } else if ans.ends_with(ch) { 1 } else { 2.min(n) };
        if use_cnt == 0 {
            if let Some(Item(ch2, n2)) = heap.pop() {
                ans.push(ch2);
                if n2 > 1 {
                    heap.push(Item(ch2, n2 - 1));
                }
                heap.push(Item(ch, n));
            } else {
                break;
            }
        } else {
            for _ in 0..use_cnt {
                ans.push(ch);
            }
            n -= use_cnt;
            if n > 0 {
                heap.push(Item(ch, n));
            }
        }
    }
    ans
}

fn main() {
    println!("{}", longest_diverse_string(1, 1, 7));
}

#[cfg(test)]
mod tests {
    use super::longest_diverse_string;

    #[test]
    fn example_one() {
        assert_eq!(longest_diverse_string(1, 1, 7), "ccaccbcc");
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_diverse_string(7, 1, 0), "aabaa");
    }
}

