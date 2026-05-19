/// LeetCode #1103 - Distribute Candies to People
fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
    let n = num_people as usize;
    let mut ans = vec![0i32; n];
    let mut c = candies as i64;
    let mut round = 0i64;
    while c > 0 {
        for i in 0..n {
            let give = (round * n as i64 + i as i64 + 1).min(c);
            ans[i] += give as i32;
            c -= give;
            if c == 0 {
                break;
            }
        }
        round += 1;
    }
    ans
}

fn main() {
    println!("{:?}", distribute_candies(7, 4));
}

#[cfg(test)]
mod tests {
    use super::distribute_candies;

    #[test]
    fn example_one() {
        assert_eq!(distribute_candies(7, 4), vec![1, 2, 3, 1]);
    }

    #[test]
    fn example_two() {
        assert_eq!(distribute_candies(10, 3), vec![5, 2, 3]);
    }
}
