/// LeetCode #770 - Basic Calculator IV
use std::collections::HashMap;

type Term = Vec<String>;

#[derive(Clone, Default)]
struct Poly {
    coeff: HashMap<Term, i64>,
}

impl Poly {
    fn from_const(c: i64) -> Self {
        let mut coeff = HashMap::new();
        if c != 0 {
            coeff.insert(vec![], c);
        }
        Poly { coeff }
    }

    fn from_var(v: String) -> Self {
        let mut coeff = HashMap::new();
        coeff.insert(vec![v], 1);
        Poly { coeff }
    }

    fn add_assign(&mut self, other: &Poly, sign: i64) {
        for (k, c) in &other.coeff {
            *self.coeff.entry(k.clone()).or_insert(0) += sign * c;
        }
        self.coeff.retain(|_, c| *c != 0);
    }

    fn mul(&self, other: &Poly) -> Poly {
        let mut coeff = HashMap::new();
        for (k1, c1) in &self.coeff {
            for (k2, c2) in &other.coeff {
                let mut k = k1.clone();
                k.extend(k2.iter().cloned());
                k.sort();
                *coeff.entry(k).or_insert(0) += c1 * c2;
            }
        }
        coeff.retain(|_, c| *c != 0);
        Poly { coeff }
    }

    fn to_list(&self) -> Vec<String> {
        let mut terms: Vec<(&Term, i64)> = self
            .coeff
            .iter()
            .filter(|(_, c)| **c != 0)
            .map(|(k, c)| (k, *c))
            .collect();
        terms.sort_by(|a, b| b.0.len().cmp(&a.0.len()).then_with(|| a.0.cmp(b.0)));
        terms
            .into_iter()
            .map(|(vars, c)| {
                if vars.is_empty() {
                    c.to_string()
                } else {
                    format!("{}*{}", c, vars.join("*"))
                }
            })
            .collect()
    }
}

struct Parser<'a> {
    s: Vec<char>,
    i: usize,
    eval: &'a HashMap<String, i64>,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<char> {
        self.s.get(self.i).copied()
    }

    fn skip_spaces(&mut self) {
        while matches!(self.peek(), Some(' ')) {
            self.i += 1;
        }
    }

    fn parse_expr(&mut self) -> Poly {
        let mut p = self.parse_term();
        loop {
            self.skip_spaces();
            match self.peek() {
                Some('+') => {
                    self.i += 1;
                    let t = self.parse_term();
                    p.add_assign(&t, 1);
                }
                Some('-') => {
                    self.i += 1;
                    let t = self.parse_term();
                    p.add_assign(&t, -1);
                }
                _ => break,
            }
        }
        p
    }

    fn parse_term(&mut self) -> Poly {
        let mut p = self.parse_factor();
        loop {
            self.skip_spaces();
            if self.peek() == Some('*') {
                self.i += 1;
                let f = self.parse_factor();
                p = p.mul(&f);
            } else {
                break;
            }
        }
        p
    }

    fn parse_factor(&mut self) -> Poly {
        self.skip_spaces();
        match self.peek() {
            Some('(') => {
                self.i += 1;
                let p = self.parse_expr();
                self.skip_spaces();
                if self.peek() == Some(')') {
                    self.i += 1;
                }
                p
            }
            Some(c) if c.is_ascii_digit() => {
                let mut v = 0i64;
                while let Some(d) = self.peek() {
                    if d.is_ascii_digit() {
                        v = v * 10 + (d as i64 - '0' as i64);
                        self.i += 1;
                    } else {
                        break;
                    }
                }
                Poly::from_const(v)
            }
            Some(c) if c.is_ascii_lowercase() => {
                let mut name = String::new();
                while let Some(d) = self.peek() {
                    if d.is_ascii_lowercase() {
                        name.push(d);
                        self.i += 1;
                    } else {
                        break;
                    }
                }
                if let Some(&val) = self.eval.get(&name) {
                    Poly::from_const(val)
                } else {
                    Poly::from_var(name)
                }
            }
            _ => Poly::from_const(0),
        }
    }
}

fn basic_calculator_iv(
    expression: String,
    evalvars: Vec<String>,
    evalints: Vec<i32>,
) -> Vec<String> {
    let eval: HashMap<String, i64> = evalvars
        .into_iter()
        .zip(evalints.into_iter())
        .map(|(k, v)| (k, v as i64))
        .collect();
    let mut p = Parser {
        s: expression.chars().collect(),
        i: 0,
        eval: &eval,
    };
    p.parse_expr().to_list()
}

fn main() {
    let out = basic_calculator_iv("e + 8 - a + 5".into(), vec!["e".into()], vec![1]);
    println!("{:?}", out);
}

#[cfg(test)]
mod tests {
    use super::basic_calculator_iv;

    #[test]
    fn example_one() {
        assert_eq!(
            basic_calculator_iv("e + 8 - a + 5".into(), vec!["e".into()], vec![1]),
            vec!["-1*a".to_string(), "14".to_string()]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            basic_calculator_iv(
                "e - 8 + temperature - pressure".into(),
                vec!["e".into(), "temperature".into()],
                vec![1, 12]
            ),
            vec!["-1*pressure".to_string(), "5".to_string()]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            basic_calculator_iv("(e + 8) * (e - 8)".into(), vec![], vec![]),
            vec!["1*e*e".to_string(), "-64".to_string()]
        );
    }
}
