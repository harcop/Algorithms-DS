/// LeetCode #3484 - Design Spreadsheet
use std::collections::HashMap;

struct Spreadsheet {
    d: HashMap<String, i32>,
}

impl Spreadsheet {
    fn new(_rows: i32) -> Self {
        Self { d: HashMap::new() }
    }

    fn set_cell(&mut self, cell: String, value: i32) {
        self.d.insert(cell, value);
    }

    fn reset_cell(&mut self, cell: String) {
        self.d.remove(&cell);
    }

    fn get_value(&self, formula: String) -> i32 {
        formula[1..]
            .split('+')
            .map(|cell| {
                if cell.as_bytes()[0].is_ascii_digit() {
                    cell.parse().unwrap()
                } else {
                    *self.d.get(cell).unwrap_or(&0)
                }
            })
            .sum()
    }
}

fn main() {
    let mut spreadsheet = Spreadsheet::new(3);
    println!("{}", spreadsheet.get_value("=5+7".into()));
    spreadsheet.set_cell("A1".into(), 10);
    println!("{}", spreadsheet.get_value("=A1+6".into()));
}

#[cfg(test)]
mod tests {
    use super::Spreadsheet;

    #[test]
    fn example1() {
        let mut spreadsheet = Spreadsheet::new(3);
        assert_eq!(spreadsheet.get_value("=5+7".into()), 12);
        spreadsheet.set_cell("A1".into(), 10);
        assert_eq!(spreadsheet.get_value("=A1+6".into()), 16);
        spreadsheet.set_cell("B2".into(), 15);
        assert_eq!(spreadsheet.get_value("=A1+B2".into()), 25);
        spreadsheet.reset_cell("A1".into());
        assert_eq!(spreadsheet.get_value("=A1+B2".into()), 15);
    }
}
