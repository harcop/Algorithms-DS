/// LeetCode #2625 - Flatten Deeply Nested Array (JS problem; Rust enum analogue)
#[derive(Clone, Debug, PartialEq)]
enum Nested {
    Int(i32),
    Arr(Vec<Nested>),
}

fn flat(arr: &[Nested], n: i32) -> Vec<Nested> {
    if n == 0 {
        return arr.to_vec();
    }
    let mut ans = Vec::new();
    for x in arr {
        match x {
            Nested::Arr(inner) if n > 0 => ans.extend(flat(inner, n - 1)),
            other => ans.push(other.clone()),
        }
    }
    ans
}

fn main() {
    let arr = vec![
        Nested::Int(1),
        Nested::Int(2),
        Nested::Arr(vec![Nested::Int(4), Nested::Int(5)]),
    ];
    println!("{:?}", flat(&arr, 1));
}

#[cfg(test)]
mod tests {
    use super::{flat, Nested};

    #[test]
    fn depth_zero() {
        let arr = vec![
            Nested::Int(1),
            Nested::Arr(vec![Nested::Int(4), Nested::Int(5)]),
        ];
        assert_eq!(flat(&arr, 0), arr);
    }

    #[test]
    fn depth_one() {
        let arr = vec![
            Nested::Int(1),
            Nested::Int(2),
            Nested::Int(3),
            Nested::Arr(vec![Nested::Int(4), Nested::Int(5), Nested::Int(6)]),
            Nested::Arr(vec![
                Nested::Int(7),
                Nested::Int(8),
                Nested::Arr(vec![Nested::Int(9), Nested::Int(10), Nested::Int(11)]),
                Nested::Int(12),
            ]),
            Nested::Arr(vec![Nested::Int(13), Nested::Int(14), Nested::Int(15)]),
        ];
        assert_eq!(
            flat(&arr, 1),
            vec![
                Nested::Int(1),
                Nested::Int(2),
                Nested::Int(3),
                Nested::Int(4),
                Nested::Int(5),
                Nested::Int(6),
                Nested::Int(7),
                Nested::Int(8),
                Nested::Arr(vec![Nested::Int(9), Nested::Int(10), Nested::Int(11)]),
                Nested::Int(12),
                Nested::Int(13),
                Nested::Int(14),
                Nested::Int(15),
            ]
        );
    }

    #[test]
    fn depth_two_fully_flattens() {
        let arr = vec![
            Nested::Arr(vec![Nested::Int(1), Nested::Int(2), Nested::Int(3)]),
            Nested::Arr(vec![
                Nested::Int(7),
                Nested::Int(8),
                Nested::Arr(vec![Nested::Int(9), Nested::Int(10)]),
                Nested::Int(12),
            ]),
        ];
        assert_eq!(
            flat(&arr, 2),
            vec![
                Nested::Int(1),
                Nested::Int(2),
                Nested::Int(3),
                Nested::Int(7),
                Nested::Int(8),
                Nested::Int(9),
                Nested::Int(10),
                Nested::Int(12),
            ]
        );
    }
}
