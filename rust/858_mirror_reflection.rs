/// LeetCode #858 - Mirror Reflection
fn mirror_reflection(p: i32, q: i32) -> i32 {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let t = a % b;
            a = b;
            b = t;
        }
        a
    }
    let g = gcd(p, q);
    let p = p / g;
    let q = q / g;
    if p % 2 == 1 {
        if q % 2 == 1 {
            1
        } else {
            0
        }
    } else if q % 2 == 1 {
        2
    } else {
        0
    }
}

fn main() {
    println!("{}", mirror_reflection(2, 1));
}

#[cfg(test)]
mod tests {
    use super::mirror_reflection;

    #[test]
    fn example_one() {
        assert_eq!(mirror_reflection(2, 1), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(mirror_reflection(3, 1), 1);
    }
}
