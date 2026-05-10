/// LeetCode #582 - Kill Process
use std::collections::{HashMap, VecDeque};

fn kill_process(pid: Vec<i32>, ppid: Vec<i32>, kill: i32) -> Vec<i32> {
    let mut g: HashMap<i32, Vec<i32>> = HashMap::new();
    for i in 0..pid.len() {
        g.entry(ppid[i]).or_default().push(pid[i]);
    }
    let mut out = vec![];
    let mut q = VecDeque::new();
    q.push_back(kill);
    while let Some(x) = q.pop_front() {
        out.push(x);
        if let Some(ch) = g.get(&x) {
            for &c in ch {
                q.push_back(c);
            }
        }
    }
    out
}

fn main() {
    println!("{:?}", kill_process(vec![1, 3, 10, 5], vec![3, 0, 5, 3], 5));
}

#[cfg(test)]
mod tests {
    use super::kill_process;

    #[test]
    fn example_one() {
        let mut v = kill_process(vec![1, 3, 10, 5], vec![3, 0, 5, 3], 5);
        v.sort();
        assert_eq!(v, vec![5, 10]);
    }
}
