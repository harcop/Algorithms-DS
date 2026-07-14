/// LeetCode #2386 - Find the K-Sum of an Array
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn k_sum(mut nums: Vec<i32>, mut k: i32) -> i64 {
    let mut mx = 0i64;
    for x in nums.iter_mut() {
        if *x > 0 {
            mx += *x as i64;
        } else {
            *x = -*x;
        }
    }
    nums.sort_unstable();
    let n = nums.len();
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0i64, 0usize)));
    while k > 1 {
        k -= 1;
        let Reverse((s, i)) = heap.pop().unwrap();
        if i < n {
            heap.push(Reverse((s + nums[i] as i64, i + 1)));
            if i > 0 {
                heap.push(Reverse((
                    s + nums[i] as i64 - nums[i - 1] as i64,
                    i + 1,
                )));
            }
        }
    }
    mx - heap.peek().unwrap().0 .0
}

fn main() {
    println!("{}", k_sum(vec![2, 4, -2], 5));
}

#[cfg(test)]
mod tests {
    use super::k_sum;

    #[test]
    fn example_one() {
        assert_eq!(k_sum(vec![2, 4, -2], 5), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(k_sum(vec![1, -2, 3, 4, -10, 12], 16), 10);
    }
}
