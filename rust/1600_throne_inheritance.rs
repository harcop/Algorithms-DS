/// LeetCode #1600 - Throne Inheritance
use std::collections::{HashMap, HashSet};

pub struct ThroneInheritance {
    king: String,
    dead: HashSet<String>,
    children: HashMap<String, Vec<String>>,
}

impl ThroneInheritance {
    fn new(king_name: String) -> Self {
        ThroneInheritance { king: king_name.clone(), dead: HashSet::new(), children: HashMap::new() }
    }
    fn birth(&mut self, parent_name: String, child_name: String) {
        self.children.entry(parent_name).or_default().push(child_name);
    }
    fn death(&mut self, name: String) { self.dead.insert(name); }
    fn get_inheritance_order(&self) -> Vec<String> {
        let mut out = vec![];
        fn dfs(u: &str, ch: &HashMap<String, Vec<String>>, dead: &HashSet<String>, out: &mut Vec<String>) {
            if !dead.contains(u) { out.push(u.to_string()); }
            if let Some(kids) = ch.get(u) {
                for k in kids { dfs(k, ch, dead, out); }
            }
        }
        dfs(&self.king, &self.children, &self.dead, &mut out);
        out
    }
}
fn main() {
    let mut t = ThroneInheritance::new("king".into());
    t.birth("king".into(), "andy".into());
    println!("{:?}", t.get_inheritance_order());
}
#[cfg(test)]
mod tests {
    use super::ThroneInheritance;
    #[test]
    fn example_one() {
        let mut t = ThroneInheritance::new("king".into());
        t.birth("king".into(), "andy".into());
        t.birth("king".into(), "bob".into());
        t.birth("king".into(), "catherine".into());
        t.birth("andy".into(), "matthew".into());
        t.birth("bob".into(), "alex".into());
        t.birth("bob".into(), "asha".into());
        assert_eq!(t.get_inheritance_order(), vec!["king","andy","matthew","bob","alex","asha","catherine"]);
        t.death("bob".into());
        assert_eq!(t.get_inheritance_order(), vec!["king","andy","matthew","alex","asha","catherine"]);
    }
}