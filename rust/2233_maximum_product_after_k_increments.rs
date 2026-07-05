/// LeetCode #2233 - Maximum Product After K Increments
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn maximum_product(nums: Vec<i32>, k: i32) -> i32 {
    let mut heap: BinaryHeap<Reverse<i32>> = nums.into_iter().map(Reverse).collect();
    for _ in 0..k {
        let Reverse(mut x) = heap.pop().unwrap();
        x += 1;
        heap.push(Reverse(x));
    }

    const MOD: i64 = 1_000_000_007;
    heap.iter()
        .fold(1i64, |acc, &Reverse(x)| acc * x as i64 % MOD) as i32
}

fn main() {
    println!("{}", maximum_product(vec![0, 4], 5));
}

#[cfg(test)]
mod tests {
    use super::maximum_product;

    #[test]
    fn example_one() {
        assert_eq!(maximum_product(vec![0, 4], 5), 20);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_product(vec![6, 3, 3, 2], 2), 216);
    }
}
