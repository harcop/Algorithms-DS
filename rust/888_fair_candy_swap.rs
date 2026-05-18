/// LeetCode #888 - Fair Candy Swap
use std::collections::HashSet;

fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
    let sa: i32 = alice_sizes.iter().sum();
    let sb: i32 = bob_sizes.iter().sum();
    let delta = sa - sb;
    let bob: HashSet<i32> = bob_sizes.iter().copied().collect();
    for x in alice_sizes {
        let y = x - delta / 2;
        if bob.contains(&y) {
            return vec![x, y];
        }
    }
    vec![]
}

fn main() {
    println!("{:?}", fair_candy_swap(vec![1, 1], vec![2, 2]));
}

#[cfg(test)]
mod tests {
    use super::fair_candy_swap;

    #[test]
    fn example_one() {
        let got = fair_candy_swap(vec![1, 1], vec![2, 2]);
        assert!(got == vec![1, 2] || got == vec![2, 1]);
    }
}
