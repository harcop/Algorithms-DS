/// LeetCode #2590 - Design a Todo List
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
struct Task {
    task_id: i32,
    description: String,
    due_date: i32,
    tags: HashSet<String>,
    finished: bool,
}

struct TodoList {
    next_id: i32,
    tasks: HashMap<i32, Vec<Task>>,
}

impl TodoList {
    fn new() -> Self {
        Self {
            next_id: 1,
            tasks: HashMap::new(),
        }
    }

    fn add_task(
        &mut self,
        user_id: i32,
        task_description: String,
        due_date: i32,
        tags: Vec<String>,
    ) -> i32 {
        let task_id = self.next_id;
        self.next_id += 1;
        self.tasks.entry(user_id).or_default().push(Task {
            task_id,
            description: task_description,
            due_date,
            tags: tags.into_iter().collect(),
            finished: false,
        });
        task_id
    }

    fn get_all_tasks(&self, user_id: i32) -> Vec<String> {
        let mut tasks: Vec<&Task> = self
            .tasks
            .get(&user_id)
            .map(|v| v.iter().filter(|t| !t.finished).collect())
            .unwrap_or_default();
        tasks.sort_by_key(|t| t.due_date);
        tasks.into_iter().map(|t| t.description.clone()).collect()
    }

    fn get_tasks_for_tag(&self, user_id: i32, tag: String) -> Vec<String> {
        let mut tasks: Vec<&Task> = self
            .tasks
            .get(&user_id)
            .map(|v| {
                v.iter()
                    .filter(|t| !t.finished && t.tags.contains(&tag))
                    .collect()
            })
            .unwrap_or_default();
        tasks.sort_by_key(|t| t.due_date);
        tasks.into_iter().map(|t| t.description.clone()).collect()
    }

    fn complete_task(&mut self, user_id: i32, task_id: i32) {
        if let Some(tasks) = self.tasks.get_mut(&user_id) {
            for task in tasks {
                if task.task_id == task_id {
                    task.finished = true;
                    break;
                }
            }
        }
    }
}

fn main() {
    let mut todo = TodoList::new();
    println!("{}", todo.add_task(1, "Task1".into(), 50, vec![]));
}

#[cfg(test)]
mod tests {
    use super::TodoList;

    #[test]
    fn example_sequence() {
        let mut todo = TodoList::new();
        assert_eq!(todo.add_task(1, "Task1".into(), 50, vec![]), 1);
        assert_eq!(
            todo.add_task(1, "Task2".into(), 100, vec!["P1".into()]),
            2
        );
        assert_eq!(
            todo.get_all_tasks(1),
            vec!["Task1".to_string(), "Task2".to_string()]
        );
        assert_eq!(todo.get_all_tasks(5), Vec::<String>::new());
        assert_eq!(
            todo.add_task(1, "Task3".into(), 30, vec!["P1".into()]),
            3
        );
        assert_eq!(
            todo.get_tasks_for_tag(1, "P1".into()),
            vec!["Task3".to_string(), "Task2".to_string()]
        );
        todo.complete_task(5, 1);
        todo.complete_task(1, 2);
        assert_eq!(
            todo.get_tasks_for_tag(1, "P1".into()),
            vec!["Task3".to_string()]
        );
        assert_eq!(
            todo.get_all_tasks(1),
            vec!["Task3".to_string(), "Task1".to_string()]
        );
    }
}
