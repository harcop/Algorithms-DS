/// LeetCode #1997 - First Day Where You Have Been in All the Rooms
const MOD: i64 = 1_000_000_007;

fn first_day_been_in_all_rooms(next_visit: Vec<i32>) -> i32 {
    let n = next_visit.len();
    let mut f = vec![0i64; n];
    for i in 1..n {
        f[i] = (2 * f[i - 1] - f[next_visit[i - 1] as usize] + 2).rem_euclid(MOD);
    }
    f[n - 1] as i32
}

fn main() {
    println!("{}", first_day_been_in_all_rooms(vec![0, 0]));
}

#[cfg(test)]
mod tests {
    use super::first_day_been_in_all_rooms;

    #[test]
    fn example_one() {
        assert_eq!(first_day_been_in_all_rooms(vec![0, 0]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(first_day_been_in_all_rooms(vec![0, 0, 2]), 6);
    }
}
