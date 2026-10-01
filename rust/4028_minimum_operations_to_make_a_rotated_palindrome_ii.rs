/// LeetCode #4028 - Minimum Operations to Make a Rotated Palindrome II
fn fft(re: &mut [f64], im: &mut [f64], inv: bool) {
    let n = re.len();
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            re.swap(i, j);
            im.swap(i, j);
        }
    }
    let mut len = 2usize;
    while len <= n {
        let ang = std::f64::consts::TAU / len as f64 * if inv { -1.0 } else { 1.0 };
        let (wr, wi) = (ang.cos(), ang.sin());
        let half = len >> 1;
        let mut i = 0usize;
        while i < n {
            let mut cr = 1.0;
            let mut ci = 0.0;
            for k in 0..half {
                let x = i + k;
                let y = x + half;
                let tr = re[y] * cr - im[y] * ci;
                let ti = re[y] * ci + im[y] * cr;
                let ur = re[x];
                let ui = im[x];
                re[x] = ur + tr;
                im[x] = ui + ti;
                re[y] = ur - tr;
                im[y] = ui - ti;
                let nr = cr * wr - ci * wi;
                let ni = cr * wi + ci * wr;
                cr = nr;
                ci = ni;
            }
            i += len;
        }
        len <<= 1;
    }
    if inv {
        let inv_n = 1.0 / n as f64;
        for i in 0..n {
            re[i] *= inv_n;
            im[i] *= inv_n;
        }
    }
}

fn min_operations(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut size = 1usize;
    while size < 2 * n {
        size <<= 1;
    }
    let nums: Vec<f64> = s.iter().map(|&c| (c - b'a') as f64).collect();
    let mut cost = [0.0f64; 26];
    for t in 0..26 {
        for z in 0..26 {
            let d = z.min(26 - z) as f64;
            cost[t] += d * (std::f64::consts::TAU * t as f64 * z as f64 / 26.0).cos();
        }
    }
    let mut dp = vec![0.0f64; n];
    let mut re = vec![0.0; size];
    let mut im = vec![0.0; size];
    let mut bre = vec![0.0; size];
    let mut bim = vec![0.0; size];
    for t in 0..14 {
        let theta = std::f64::consts::TAU * t as f64 / 26.0;
        for i in 0..n {
            let ang = theta * nums[i];
            re[i] = ang.cos();
            im[i] = ang.sin();
        }
        for i in n..size {
            re[i] = 0.0;
            im[i] = 0.0;
        }
        fft(&mut re, &mut im, false);
        for i in 0..size {
            let ar = re[i];
            let ai = im[i];
            let j = (size - i) & (size - 1);
            let br = re[j];
            let bi = -im[j];
            bre[i] = ar * br - ai * bi;
            bim[i] = -(ar * bi + ai * br);
        }
        fft(&mut bre, &mut bim, false);
        let mult = if t == 0 || t == 13 { 1.0 } else { 2.0 };
        let factor = mult * cost[t] / size as f64;
        for c in 0..n {
            dp[c] += factor * (bre[c] + bre[c + n]);
        }
    }
    let mut ans = i32::MAX;
    for k in 0..n {
        let c = (2 * k + n - 1) % n;
        let d = (dp[c] / 52.0).round() as i32;
        ans = ans.min(k as i32 + d);
    }
    ans
}

fn main() {
    println!("{}", min_operations("abc".to_string()));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    fn brute(s: &str) -> i32 {
        let b = s.as_bytes();
        let n = b.len();
        let mut ans = i32::MAX;
        for k in 0..n {
            let mut t = k as i32;
            let mut i = 0usize;
            let mut j = n - 1;
            while i < j {
                let x = b[(i + k) % n] - b'a';
                let y = b[(j + k) % n] - b'a';
                let d = (x as i32 - y as i32).abs();
                t += d.min(26 - d);
                i += 1;
                j -= 1;
            }
            ans = ans.min(t);
        }
        ans
    }

    #[test]
    fn example1() {
        assert_eq!(min_operations("abc".to_string()), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations("yb".to_string()), 3);
    }

    #[test]
    fn matches_brute_on_short_strings() {
        let samples = ["ab", "zz", "abc", "azby", "leetcode", "palindrome", "aaaa", "abcdcba"];
        for s in samples {
            assert_eq!(min_operations(s.to_string()), brute(s), "{s}");
        }
    }
}
