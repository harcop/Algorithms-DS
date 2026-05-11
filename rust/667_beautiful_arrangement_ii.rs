/// LeetCode #667 - Beautiful Arrangement II
fn construct_array(n: i32, k: i32) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::with_capacity(n as usize);
    let mut lo = 1i32;
    let mut hi = k + 1;
    let mut use_lo = true;
    while lo <= hi {
        if use_lo {
            out.push(lo);
            lo += 1;
        } else {
            out.push(hi);
            hi -= 1;
        }
        use_lo = !use_lo;
    }
    for v in (k + 2)..=n {
        out.push(v);
    }
    out
}

fn main() {
    println!("{:?}", construct_array(3, 1));
}

#[cfg(test)]
mod tests {
    use super::construct_array;

    fn distinct_count(v: &[i32]) -> usize {
        use std::collections::HashSet;
        let s: HashSet<i32> = v.windows(2).map(|w| (w[0] - w[1]).abs()).collect();
        s.len()
    }

    #[test]
    fn example_one() {
        let v = construct_array(3, 1);
        assert_eq!(v.len(), 3);
        assert_eq!(distinct_count(&v), 1);
    }

    #[test]
    fn example_two() {
        let v = construct_array(3, 2);
        assert_eq!(v.len(), 3);
        assert_eq!(distinct_count(&v), 2);
    }
}
