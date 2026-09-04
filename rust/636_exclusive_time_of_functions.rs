/// LeetCode #636 - Exclusive Time of Functions
fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
    let mut ans = vec![0; n as usize];
    let mut stack: Vec<i32> = vec![];
    let mut prev = 0i32;
    for log in logs {
        let parts: Vec<&str> = log.split(':').collect();
        let id: i32 = parts[0].parse().unwrap();
        let time: i32 = parts[2].parse().unwrap();
        if parts[1] == "start" {
            if let Some(&top) = stack.last() {
                ans[top as usize] += time - prev;
            }
            stack.push(id);
            prev = time;
        } else {
            let top = stack.pop().unwrap();
            ans[top as usize] += time - prev + 1;
            prev = time + 1;
        }
    }
    ans
}

fn main() {
    let logs = vec![
        "0:start:0".into(),
        "1:start:2".into(),
        "1:end:5".into(),
        "0:end:6".into(),
    ];
    println!("{:?}", exclusive_time(2, logs));
}

#[cfg(test)]
mod tests {
    use super::exclusive_time;

    #[test]
    fn example_one() {
        let logs = vec![
            "0:start:0".into(),
            "1:start:2".into(),
            "1:end:5".into(),
            "0:end:6".into(),
        ];
        assert_eq!(exclusive_time(2, logs), vec![3, 4]);
    }

    #[test]
    fn example_two() {
        let logs = vec![
            "0:start:0".into(),
            "0:start:2".into(),
            "0:end:5".into(),
            "0:start:6".into(),
            "0:end:6".into(),
            "0:end:7".into(),
        ];
        assert_eq!(exclusive_time(1, logs), vec![8]);
    }

    #[test]
    fn example_three() {
        let logs = vec![
            "0:start:0".into(),
            "0:start:2".into(),
            "0:end:5".into(),
            "1:start:6".into(),
            "1:end:6".into(),
            "0:end:7".into(),
        ];
        assert_eq!(exclusive_time(2, logs), vec![7, 1]);
    }
}
