/// LeetCode #1882 - Process Tasks Using Servers
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
    let mut idle: BinaryHeap<Reverse<(i32, i32)>> =
        servers.iter().enumerate().map(|(i, &s)| Reverse((s, i as i32))).collect();
    let mut busy: BinaryHeap<Reverse<(i32, i32, i32)>> = BinaryHeap::new();
    let mut ans = Vec::with_capacity(tasks.len());
    for (j, &t) in tasks.iter().enumerate() {
        let j = j as i32;
        while busy.peek().map_or(false, |Reverse((w, _, _))| *w <= j) {
            let Reverse((_, s, i)) = busy.pop().unwrap();
            idle.push(Reverse((s, i)));
        }
        if let Some(Reverse((s, i))) = idle.pop() {
            busy.push(Reverse((j + t, s, i)));
            ans.push(i);
        } else {
            let Reverse((w, s, i)) = busy.pop().unwrap();
            busy.push(Reverse((w + t, s, i)));
            ans.push(i);
        }
    }
    ans
}

fn main() {
    println!("{:?}", assign_tasks(vec![3, 3, 2], vec![1, 2, 3, 2, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::assign_tasks;

    #[test]
    fn example_one() {
        assert_eq!(
            assign_tasks(vec![3, 3, 2], vec![1, 2, 3, 2, 1, 2]),
            vec![2, 2, 0, 2, 1, 2]
        );
    }
}
