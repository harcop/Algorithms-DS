/// LeetCode #1040 - Moving Stones Until Consecutive II
fn num_moves_stones_ii(stones: Vec<i32>) -> Vec<i32> {
    let mut stones = stones;
    stones.sort_unstable();
    let n = stones.len();
    let mut min_moves = n as i32;
    let mut j = 0usize;
    for i in 0..n {
        while stones[i] - stones[j] + 1 > n as i32 {
            j += 1;
        }
        let mut need = (n - (i - j + 1)) as i32;
        if need == 1 && stones[i] - stones[j] + 1 == n as i32 - 1 {
            need = 2;
        }
        min_moves = min_moves.min(need);
    }
    let max_moves = (stones[n - 1] - stones[1] - n as i32 + 2)
        .max(stones[n - 2] - stones[0] - n as i32 + 2);
    vec![min_moves, max_moves]
}

fn main() {
    println!("{:?}", num_moves_stones_ii(vec![7, 4, 9]));
}

#[cfg(test)]
mod tests {
    use super::num_moves_stones_ii;

    #[test]
    fn example_one() {
        assert_eq!(num_moves_stones_ii(vec![7, 4, 9]), vec![1, 2]);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_moves_stones_ii(vec![6, 5, 4, 3, 10]), vec![2, 3]);
    }
}
