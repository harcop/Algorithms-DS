/// LeetCode #3588 - Find Maximum Area of a Triangle
use std::collections::HashMap;

fn calc(coords: &[Vec<i32>]) -> i64 {
    let mut mn = i32::MAX;
    let mut mx = 0;
    let mut f: HashMap<i32, i32> = HashMap::new();
    let mut g: HashMap<i32, i32> = HashMap::new();
    for c in coords {
        let (x, y) = (c[0], c[1]);
        mn = mn.min(x);
        mx = mx.max(x);
        f.entry(x).and_modify(|v| *v = (*v).min(y)).or_insert(y);
        g.entry(x).and_modify(|v| *v = (*v).max(y)).or_insert(y);
    }
    let mut ans = 0i64;
    for (&x, &y) in &f {
        let d = g[&x] - y;
        ans = ans.max(d as i64 * (mx - x).max(x - mn) as i64);
    }
    ans
}

fn max_area(mut coords: Vec<Vec<i32>>) -> i64 {
    let mut ans = calc(&coords);
    for c in &mut coords {
        c.swap(0, 1);
    }
    ans = ans.max(calc(&coords));
    if ans > 0 { ans } else { -1 }
}

fn main() {
    println!("{}", max_area(vec![vec![1, 1], vec![1, 2], vec![3, 2], vec![3, 3]]));
}

#[cfg(test)]
mod tests {
    use super::max_area;

    #[test]
    fn example1() {
        assert_eq!(
            max_area(vec![vec![1, 1], vec![1, 2], vec![3, 2], vec![3, 3]]),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(max_area(vec![vec![1, 1], vec![2, 2], vec![3, 3]]), -1);
    }
}
