/// LeetCode #385 - Mini Parser
#[derive(Debug, Clone, PartialEq, Eq)] pub enum NestedInteger { Int(i32), List(Vec<NestedInteger>) }

fn deserialize(s: String) -> NestedInteger {
    let b = s.as_bytes();
    fn parse(i: &mut usize, b: &[u8]) -> NestedInteger {
        if b[*i] == b'[' {
            *i += 1;
            let mut v = vec![];
            loop {
                match b[*i] {
                    b']' => { *i += 1; break; }
                    b',' => { *i += 1; }
                    _ => v.push(parse(i, b)),
                }
            }
            NestedInteger::List(v)
        } else {
            let mut sign = 1i32;
            if b[*i]==b'-' { sign=-1; *i+=1; }
            let mut n = 0i32;
            while *i<b.len() && b[*i].is_ascii_digit() { n=n*10+(b[*i]-b'0')as i32; *i+=1; }
            NestedInteger::Int(n*sign)
        }
    }
    let mut i=0usize; parse(&mut i, b)
}

fn main() { println!("{:?}", deserialize("324".into())); }

#[cfg(test)] mod tests { use super::*;
    #[test] fn ints() {
        match deserialize("324".into()) { NestedInteger::Int(x)=>assert_eq!(x,324), _=>panic!()}
    }
    #[test] fn list(){ match deserialize("[123,[456,[789]]]".into()) { NestedInteger::List(_)=>{}, _=>panic!() }
}}
