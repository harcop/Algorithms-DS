/// LeetCode #2632 - Curry (JS problem; Rust accumulating-args analogue)
struct Curried {
    arity: usize,
    args: Vec<i32>,
    fn_: fn(&[i32]) -> i32,
}

enum CurryResult {
    Value(i32),
    Next(Curried),
}

impl Curried {
    fn new(arity: usize, fn_: fn(&[i32]) -> i32) -> Self {
        Curried {
            arity,
            args: Vec::new(),
            fn_,
        }
    }

    fn call(mut self, next: &[i32]) -> CurryResult {
        self.args.extend_from_slice(next);
        if self.args.len() >= self.arity {
            CurryResult::Value((self.fn_)(&self.args[..self.arity]))
        } else {
            CurryResult::Next(self)
        }
    }
}

fn sum3(args: &[i32]) -> i32 {
    args.iter().sum()
}

fn life(_: &[i32]) -> i32 {
    42
}

fn main() {
    let c = Curried::new(3, sum3);
    match c.call(&[1]) {
        CurryResult::Next(c) => match c.call(&[2]) {
            CurryResult::Next(c) => match c.call(&[3]) {
                CurryResult::Value(v) => println!("{v}"),
                _ => {}
            },
            _ => {}
        },
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn apply(inputs: &[&[i32]], arity: usize, fn_: fn(&[i32]) -> i32) -> i32 {
        let mut cur = Curried::new(arity, fn_);
        let mut last = 0;
        for chunk in inputs {
            match cur.call(chunk) {
                CurryResult::Value(v) => {
                    last = v;
                    break;
                }
                CurryResult::Next(next) => cur = next,
            }
        }
        last
    }

    #[test]
    fn example_one() {
        assert_eq!(apply(&[&[1], &[2], &[3]], 3, sum3), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(apply(&[&[1, 2], &[3]], 3, sum3), 6);
    }

    #[test]
    fn example_three() {
        assert_eq!(apply(&[&[], &[], &[1, 2, 3]], 3, sum3), 6);
    }

    #[test]
    fn example_four() {
        assert_eq!(apply(&[&[]], 0, life), 42);
    }
}
