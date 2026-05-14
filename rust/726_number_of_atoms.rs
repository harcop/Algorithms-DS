/// LeetCode #726 - Number of Atoms
use std::collections::BTreeMap;

fn count_of_atoms(formula: String) -> String {
    let s = formula.into_bytes();
    let mut i = 0usize;

    fn num(s: &[u8], i: &mut usize) -> i32 {
        if *i >= s.len() || !s[*i].is_ascii_digit() {
            return 1;
        }
        let mut v = 0i32;
        while *i < s.len() && s[*i].is_ascii_digit() {
            v = v * 10 + (s[*i] - b'0') as i32;
            *i += 1;
        }
        v
    }

    fn dfs(s: &[u8], i: &mut usize) -> BTreeMap<String, i32> {
        let mut m = BTreeMap::new();
        while *i < s.len() && s[*i] != b')' {
            if s[*i] == b'(' {
                *i += 1;
                let inner = dfs(s, i);
                *i += 1;
                let k = num(s, i);
                for (name, c) in inner {
                    *m.entry(name).or_insert(0) += c * k;
                }
            } else {
                let mut name = String::new();
                name.push(s[*i] as char);
                *i += 1;
                while *i < s.len() && s[*i].is_ascii_lowercase() {
                    name.push(s[*i] as char);
                    *i += 1;
                }
                let k = num(s, i);
                *m.entry(name).or_insert(0) += k;
            }
        }
        m
    }

    let map = dfs(&s, &mut i);
    let mut out = String::new();
    for (name, c) in map {
        out.push_str(&name);
        if c > 1 {
            out.push_str(&c.to_string());
        }
    }
    out
}

fn main() {
    println!("{}", count_of_atoms("H2O".into()));
}

#[cfg(test)]
mod tests {
    use super::count_of_atoms;

    #[test]
    fn example_one() {
        assert_eq!(count_of_atoms("H2O".into()), "H2O");
    }

    #[test]
    fn example_two() {
        assert_eq!(count_of_atoms("Mg(OH)2".into()), "H2MgO2");
    }

    #[test]
    fn example_three() {
        assert_eq!(count_of_atoms("K4(ON(SO3)2)2".into()), "K4N2O14S4");
    }
}
