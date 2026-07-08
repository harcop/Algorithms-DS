/// LeetCode #2296 - Design a Text Editor
struct TextEditor {
    left: Vec<u8>,
    right: Vec<u8>,
}

#[allow(dead_code)]
impl TextEditor {
    fn new() -> Self {
        Self {
            left: Vec::new(),
            right: Vec::new(),
        }
    }

    fn add_text(&mut self, text: String) {
        self.left.extend(text.bytes());
    }

    fn delete_text(&mut self, k: i32) -> i32 {
        let k = k as usize;
        let deleted = k.min(self.left.len());
        self.left.truncate(self.left.len() - deleted);
        deleted as i32
    }

    fn cursor_left(&mut self, k: i32) -> String {
        for _ in 0..k {
            if let Some(c) = self.left.pop() {
                self.right.push(c);
            } else {
                break;
            }
        }
        self.last_10()
    }

    fn cursor_right(&mut self, k: i32) -> String {
        for _ in 0..k {
            if let Some(c) = self.right.pop() {
                self.left.push(c);
            } else {
                break;
            }
        }
        self.last_10()
    }

    fn last_10(&self) -> String {
        let start = self.left.len().saturating_sub(10);
        String::from_utf8_lossy(&self.left[start..]).into_owned()
    }
}

fn main() {
    let mut ed = TextEditor::new();
    ed.add_text("leetcode".to_string());
    println!("{}", ed.delete_text(4));
}

#[cfg(test)]
mod tests {
    use super::TextEditor;

    #[test]
    fn basic_flow() {
        let mut ed = TextEditor::new();
        ed.add_text("leetcode".to_string());
        assert_eq!(ed.delete_text(4), 4);
        ed.add_text("practice".to_string());
        assert_eq!(ed.cursor_right(3), "etpractice");
        assert_eq!(ed.cursor_left(8), "leet");
        assert_eq!(ed.delete_text(10), 4);
        assert_eq!(ed.cursor_left(2), String::new());
        assert_eq!(ed.cursor_right(6), "practi");
    }
}
