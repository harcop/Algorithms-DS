/// LeetCode #2071 - Maximum Number of Tasks You Can Assign
use std::collections::BTreeMap;

fn max_task_assign(tasks: Vec<i32>, workers: Vec<i32>, pills: i32, strength: i32) -> i32 {
    let mut tasks = tasks;
    let mut workers = workers;
    tasks.sort_unstable();
    workers.sort_unstable();

    let mut lo = 0usize;
    let mut hi = tasks.len().min(workers.len());
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        if can_assign(mid, &tasks, &workers, pills, strength) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo as i32
}

fn can_assign(k: usize, tasks: &[i32], workers: &[i32], pills: i32, strength: i32) -> bool {
    let mut available = BTreeMap::new();
    for &worker in &workers[workers.len() - k..] {
        *available.entry(worker).or_insert(0) += 1;
    }

    let mut pills_left = pills;
    for &task in tasks[..k].iter().rev() {
        let strongest = *available.keys().next_back().unwrap();
        if strongest >= task {
            remove_one(&mut available, strongest);
        } else {
            if pills_left == 0 {
                return false;
            }
            let need = task - strength;
            let Some((&worker, _)) = available.range(need..).next() else {
                return false;
            };
            remove_one(&mut available, worker);
            pills_left -= 1;
        }
    }
    true
}

fn remove_one(map: &mut BTreeMap<i32, i32>, key: i32) {
    let count = map.get_mut(&key).unwrap();
    *count -= 1;
    if *count == 0 {
        map.remove(&key);
    }
}

fn main() {
    println!("{}", max_task_assign(vec![3, 2, 1], vec![0, 3, 3], 1, 1));
}

#[cfg(test)]
mod tests {
    use super::max_task_assign;

    #[test]
    fn example_one() {
        assert_eq!(max_task_assign(vec![3, 2, 1], vec![0, 3, 3], 1, 1), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_task_assign(vec![5, 4], vec![0, 0, 0], 1, 5), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(
            max_task_assign(vec![10, 15, 30], vec![0, 10, 10, 10, 10], 3, 10),
            2
        );
    }
}
