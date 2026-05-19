/// LeetCode #1093 - Statistics from a Large Sample
fn sample_stats(count: Vec<i32>) -> Vec<f64> {
    let mut min = -1i32;
    let mut max = -1i32;
    let mut total: i64 = 0;
    let mut n: i64 = 0;
    let mut vals = Vec::new();
    for (v, &c) in count.iter().enumerate() {
        if c == 0 {
            continue;
        }
        let v = v as i32;
        if min == -1 {
            min = v;
        }
        max = v;
        total += v as i64 * c as i64;
        n += c as i64;
        for _ in 0..c {
            vals.push(v);
        }
    }
    let mean = total as f64 / n as f64;
    let median = if n % 2 == 1 {
        vals[(n as usize) / 2] as f64
    } else {
        (vals[n as usize / 2 - 1] + vals[n as usize / 2]) as f64 / 2.0
    };
    let mut mode = 0usize;
    let mut mode_cnt = 0i32;
    for (v, &c) in count.iter().enumerate() {
        if c > mode_cnt {
            mode_cnt = c;
            mode = v;
        }
    }
    vec![min as f64, max as f64, mean, median, mode as f64]
}

fn main() {
    let mut c = vec![0i32; 256];
    c[3] = 4;
    println!("{:?}", sample_stats(c));
}

#[cfg(test)]
mod tests {
    use super::sample_stats;

    fn approx(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-5, "{} vs {}", a, b);
    }

    #[test]
    fn example_one() {
        let mut c = vec![0i32; 256];
        c[3] = 4;
        let r = sample_stats(c);
        approx(r[0], 3.0);
        approx(r[1], 3.0);
        approx(r[2], 3.0);
        approx(r[3], 3.0);
        approx(r[4], 3.0);
    }
}
