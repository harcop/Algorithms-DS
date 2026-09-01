/// LeetCode #3549 - Multiply Two Polynomials
fn cmul(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
    (a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0)
}

fn fft(a: &mut [(f64, f64)], invert: bool) {
    let n = a.len();
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            a.swap(i, j);
        }
    }
    let mut len = 2usize;
    while len <= n {
        let ang = 2.0 * std::f64::consts::PI / (len as f64) * if invert { -1.0 } else { 1.0 };
        let wlen = (ang.cos(), ang.sin());
        for i in (0..n).step_by(len) {
            let mut w = (1.0, 0.0);
            for j in i..i + len / 2 {
                let u = a[j];
                let v = cmul(a[j + len / 2], w);
                a[j] = (u.0 + v.0, u.1 + v.1);
                a[j + len / 2] = (u.0 - v.0, u.1 - v.1);
                w = cmul(w, wlen);
            }
        }
        len <<= 1;
    }
    if invert {
        for x in a.iter_mut() {
            x.0 /= n as f64;
            x.1 /= n as f64;
        }
    }
}

fn multiply(poly1: Vec<i32>, poly2: Vec<i32>) -> Vec<i64> {
    if poly1.is_empty() || poly2.is_empty() {
        return vec![];
    }
    let m = poly1.len() + poly2.len() - 1;
    let mut n = 1usize;
    while n < m {
        n <<= 1;
    }
    let mut fa = vec![(0.0, 0.0); n];
    let mut fb = vec![(0.0, 0.0); n];
    for i in 0..poly1.len() {
        fa[i] = (poly1[i] as f64, 0.0);
    }
    for i in 0..poly2.len() {
        fb[i] = (poly2[i] as f64, 0.0);
    }
    fft(&mut fa, false);
    fft(&mut fb, false);
    for i in 0..n {
        fa[i] = cmul(fa[i], fb[i]);
    }
    fft(&mut fa, true);
    (0..m).map(|i| fa[i].0.round() as i64).collect()
}

fn main() {
    println!("{:?}", multiply(vec![3, 2, 5], vec![1, 4]));
}

#[cfg(test)]
mod tests {
    use super::multiply;

    #[test]
    fn example1() {
        assert_eq!(multiply(vec![3, 2, 5], vec![1, 4]), vec![3, 14, 13, 20]);
    }

    #[test]
    fn example2() {
        assert_eq!(multiply(vec![1, 0, -2], vec![-1]), vec![-1, 0, 2]);
    }

    #[test]
    fn example3() {
        assert_eq!(multiply(vec![1, 5, -3], vec![-4, 2, 0]), vec![-4, -18, 22, -6, 0]);
    }
}
