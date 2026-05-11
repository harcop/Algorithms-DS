/// LeetCode #621 - Task Scheduler
fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    let mut cnt = [0i32; 26];
    for c in tasks.iter() {
        cnt[(*c as u8 - b'A') as usize] += 1;
    }
    let max_count = *cnt.iter().max().unwrap();
    let ties = cnt.iter().filter(|&&c| c == max_count).count() as i32;
    let len = tasks.len() as i32;
    ((max_count - 1) * (n + 1) + ties).max(len)
}

fn main() {
    println!("{}", least_interval(vec!['A','A','A','B','B','B'], 2));
}

#[cfg(test)]
mod tests {
    use super::least_interval;

    #[test]
    fn example_one() {
        assert_eq!(least_interval(vec!['A','A','A','B','B','B'], 2), 8);
    }

    #[test]
    fn example_two() {
        assert_eq!(least_interval(vec!['A','A','A','B','B','B'], 0), 6);
    }
}
