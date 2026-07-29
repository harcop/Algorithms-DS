/// LeetCode #2759 - Convert JSON String to Object (JS problem; Rust analogue)
#[derive(Debug, PartialEq)]
enum Json {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

struct Parser {
    chars: Vec<char>,
    pos: usize,
}

impl Parser {
    fn new(s: &str) -> Self {
        Parser { chars: s.chars().collect(), pos: 0 }
    }

    fn peek(&self) -> char {
        self.chars[self.pos]
    }

    fn parse_value(&mut self) -> Json {
        match self.peek() {
            '{' => self.parse_object(),
            '[' => self.parse_array(),
            '"' => Json::Str(self.parse_string()),
            't' => { self.pos += 4; Json::Bool(true) }
            'f' => { self.pos += 5; Json::Bool(false) }
            'n' => { self.pos += 4; Json::Null }
            _ => self.parse_number(),
        }
    }

    fn parse_string(&mut self) -> String {
        self.pos += 1;
        let mut s = String::new();
        while self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            self.pos += 1;
            if c == '"' { break; }
            s.push(c);
        }
        s
    }

    fn parse_number(&mut self) -> Json {
        let start = self.pos;
        while self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            if c == ',' || c == '}' || c == ']' { break; }
            self.pos += 1;
        }
        let s: String = self.chars[start..self.pos].iter().collect();
        Json::Number(s.parse().unwrap())
    }

    fn parse_array(&mut self) -> Json {
        self.pos += 1;
        let mut arr = Vec::new();
        while self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            if c == ']' { self.pos += 1; break; }
            if c == ',' { self.pos += 1; continue; }
            arr.push(self.parse_value());
        }
        Json::Array(arr)
    }

    fn parse_object(&mut self) -> Json {
        self.pos += 1;
        let mut obj = Vec::new();
        while self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            if c == '}' { self.pos += 1; break; }
            if c == ',' { self.pos += 1; continue; }
            let key = self.parse_string();
            self.pos += 1; // skip ':'
            let val = self.parse_value();
            obj.push((key, val));
        }
        Json::Object(obj)
    }
}

fn json_parse(s: &str) -> Json {
    Parser::new(s).parse_value()
}

fn main() {
    println!("{:?}", json_parse(r#"{"a":2,"b":[1,2,3]}"#));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_object() {
        let result = json_parse(r#"{"a":2}"#);
        assert_eq!(result, Json::Object(vec![("a".into(), Json::Number(2.0))]));
    }

    #[test]
    fn example_bool() {
        assert_eq!(json_parse("true"), Json::Bool(true));
        assert_eq!(json_parse("false"), Json::Bool(false));
    }

    #[test]
    fn example_null() {
        assert_eq!(json_parse("null"), Json::Null);
    }

    #[test]
    fn example_array() {
        assert_eq!(
            json_parse("[1,2,3]"),
            Json::Array(vec![Json::Number(1.0), Json::Number(2.0), Json::Number(3.0)])
        );
    }
}
