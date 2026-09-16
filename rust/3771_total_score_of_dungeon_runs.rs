/// LeetCode #3771 - Total Score of Dungeon Runs
fn total_score(hp: i32, damage: Vec<i32>, requirement: Vec<i32>) -> i64 {
    let n = damage.len();
    let mut prefix = vec![0i64; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + damage[i] as i64;
    }
    let hp = hp as i64;
    let mut ans = 0i64;
    for i in 0..n {
        let target = prefix[i + 1] + requirement[i] as i64 - hp;
        let j = prefix.partition_point(|&x| x < target);
        if j <= i {
            ans += (i - j + 1) as i64;
        }
    }
    ans
}

fn main() {
    println!("{}", total_score(11, vec![3, 6, 7], vec![4, 2, 5]));
}

#[cfg(test)]
mod tests {
    use super::total_score;

    #[test]
    fn example1() {
        assert_eq!(total_score(11, vec![3, 6, 7], vec![4, 2, 5]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(total_score(2, vec![10000, 1], vec![1, 1]), 1);
    }
}
