/// LeetCode #3408 - Design Task Manager
use std::cmp::Reverse;
use std::collections::{BTreeSet, HashMap};

struct TaskManager {
    d: HashMap<i32, (i32, i32)>,
    st: BTreeSet<(Reverse<i32>, Reverse<i32>)>,
}

impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut tm = TaskManager {
            d: HashMap::new(),
            st: BTreeSet::new(),
        };
        for t in tasks {
            tm.add(t[0], t[1], t[2]);
        }
        tm
    }

    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.d.insert(task_id, (user_id, priority));
        self.st.insert((Reverse(priority), Reverse(task_id)));
    }

    fn edit(&mut self, task_id: i32, new_priority: i32) {
        let (user_id, priority) = self.d[&task_id];
        self.st.remove(&(Reverse(priority), Reverse(task_id)));
        self.d.insert(task_id, (user_id, new_priority));
        self.st.insert((Reverse(new_priority), Reverse(task_id)));
    }

    fn rmv(&mut self, task_id: i32) {
        let (_, priority) = self.d.remove(&task_id).unwrap();
        self.st.remove(&(Reverse(priority), Reverse(task_id)));
    }

    fn exec_top(&mut self) -> i32 {
        let Some(&(Reverse(_), Reverse(task_id))) = self.st.iter().next() else {
            return -1;
        };
        self.st.pop_first();
        let (user_id, _) = self.d.remove(&task_id).unwrap();
        user_id
    }
}

fn main() {
    let mut tm = TaskManager::new(vec![
        vec![1, 101, 10],
        vec![2, 102, 20],
        vec![3, 103, 15],
    ]);
    tm.add(4, 104, 5);
    println!("{}", tm.exec_top());
}

#[cfg(test)]
mod tests {
    use super::TaskManager;

    #[test]
    fn example1() {
        let mut tm = TaskManager::new(vec![
            vec![1, 101, 10],
            vec![2, 102, 20],
            vec![3, 103, 15],
        ]);
        tm.add(4, 104, 5);
        tm.edit(102, 8);
        assert_eq!(tm.exec_top(), 3);
        tm.rmv(101);
        tm.add(5, 105, 15);
        assert_eq!(tm.exec_top(), 5);
    }
}
