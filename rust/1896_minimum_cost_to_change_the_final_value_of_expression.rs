/// LeetCode #1896 - Minimum Cost to Change the Final Value of Expression
#[derive(Clone, Copy)]
struct Frame {
    c0: i32,
    c1: i32,
    op: u8,
}

fn min_operations_to_flip(expression: String) -> i32 {
    let mut stack = vec![Frame {
        c0: 0,
        c1: 0,
        op: 0,
    }];

    for e in expression.bytes() {
        if e == b'(' {
            stack.push(Frame {
                c0: 0,
                c1: 0,
                op: 0,
            });
        } else if e == b'&' || e == b'|' {
            stack.last_mut().unwrap().op = e;
        } else {
            let (r0, r1) = if e == b'0' {
                (0, 1)
            } else if e == b'1' {
                (1, 0)
            } else {
                let f = stack.pop().unwrap();
                (f.c0, f.c1)
            };
            let left = stack.pop().unwrap();
            let (c0, c1) = match left.op {
                b'&' => (
                    left.c0.min(r0),
                    (left.c1 + r1).min(left.c1.min(r1) + 1),
                ),
                b'|' => (
                    (left.c0 + r0).min(left.c0.min(r0) + 1),
                    left.c1.min(r1),
                ),
                _ => (r0, r1),
            };
            stack.push(Frame { c0, c1, op: 0 });
        }
    }
    let top = stack.last().unwrap();
    top.c0.max(top.c1)
}

fn main() {
    println!("{}", min_operations_to_flip("1&(0|1)".into()));
}

#[cfg(test)]
mod tests {
    use super::min_operations_to_flip;

    #[test]
    fn example_one() {
        assert_eq!(min_operations_to_flip("1&(0|1)".into()), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_operations_to_flip("(0&0)&(0&0&0)".into()), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(min_operations_to_flip("(0|(1|0&1))".into()), 1);
    }
}
