/// LeetCode #1010 - Pairs of Songs With Total Durations Divisible by 60
fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
    let mut cnt = [0i64; 60];
    let mut ans = 0i64;
    for t in time {
        let rem = (t % 60) as usize;
        let need = if rem == 0 { 0 } else { 60 - rem };
        ans += cnt[need];
        cnt[rem] += 1;
    }
    ans as i32
}

fn main() {
    println!("{}", num_pairs_divisible_by60(vec![30, 20, 150, 100, 40]));
}

#[cfg(test)]
mod tests {
    use super::num_pairs_divisible_by60;

    #[test]
    fn example_one() {
        assert_eq!(num_pairs_divisible_by60(vec![30, 20, 150, 100, 40]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_pairs_divisible_by60(vec![60, 60, 60]), 3);
    }
}
