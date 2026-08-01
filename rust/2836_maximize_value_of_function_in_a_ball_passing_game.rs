/// LeetCode #2836 - Maximize Value of Function in a Ball Passing Game
fn get_max_function_value(receiver: Vec<i32>, k: i64) -> i64 {
    let n = receiver.len();
    let levels = (64 - k.leading_zeros() as usize).max(1);
    let mut next = vec![vec![0usize; levels]; n];
    let mut sum = vec![vec![0i64; levels]; n];

    for (i, &receiver_index) in receiver.iter().enumerate() {
        next[i][0] = receiver_index as usize;
        sum[i][0] = i as i64;
    }
    for level in 1..levels {
        for i in 0..n {
            let middle = next[i][level - 1];
            next[i][level] = next[middle][level - 1];
            sum[i][level] = sum[i][level - 1] + sum[middle][level - 1];
        }
    }

    let mut answer = 0;
    for start in 0..n {
        let mut position = start;
        let mut value = 0;
        for level in 0..levels {
            if (k >> level) & 1 == 1 {
                value += sum[position][level];
                position = next[position][level];
            }
        }
        answer = answer.max(value + position as i64);
    }
    answer
}

fn main() {
    println!("{}", get_max_function_value(vec![2, 0, 1], 4));
}

#[cfg(test)]
mod tests {
    use super::get_max_function_value;

    #[test]
    fn examples() {
        assert_eq!(get_max_function_value(vec![2, 0, 1], 4), 6);
        assert_eq!(get_max_function_value(vec![1, 1, 1, 2, 3], 3), 10);
    }
}
