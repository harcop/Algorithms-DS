/// LeetCode #2408 - Design SQL
use std::collections::HashMap;

struct Table {
    rows: Vec<Option<Vec<String>>>,
}

struct SQL {
    tables: HashMap<String, Table>,
}

impl SQL {
    fn new(names: Vec<String>, _columns: Vec<i32>) -> Self {
        let mut tables = HashMap::new();
        for name in names {
            tables.insert(name, Table { rows: Vec::new() });
        }
        Self { tables }
    }

    fn insert_row(&mut self, name: String, row: Vec<String>) {
        self.tables.get_mut(&name).unwrap().rows.push(Some(row));
    }

    fn delete_row(&mut self, name: String, row_id: i32) {
        if let Some(slot) = self.tables.get_mut(&name).unwrap().rows.get_mut(row_id as usize - 1) {
            *slot = None;
        }
    }

    fn select_cell(&self, name: String, row_id: i32, column_id: i32) -> String {
        self.tables[&name].rows[row_id as usize - 1]
            .as_ref()
            .unwrap()[column_id as usize - 1]
            .clone()
    }
}

fn main() {
    let mut sql = SQL::new(vec!["users".to_string()], vec![2]);
    sql.insert_row("users".to_string(), vec!["alice".to_string(), "admin".to_string()]);
    println!("{}", sql.select_cell("users".to_string(), 1, 2));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn basic_flow() {
        let mut sql = SQL::new(vec!["users".to_string(), "posts".to_string()], vec![2, 3]);
        sql.insert_row("users".to_string(), vec!["1".to_string(), "alice".to_string()]);
        sql.insert_row("users".to_string(), vec!["2".to_string(), "bob".to_string()]);
        assert_eq!(sql.select_cell("users".to_string(), 2, 2), "bob");
        sql.delete_row("users".to_string(), 1);
        sql.insert_row("users".to_string(), vec!["3".to_string(), "carol".to_string()]);
        assert_eq!(sql.select_cell("users".to_string(), 3, 1), "3");
    }
}
