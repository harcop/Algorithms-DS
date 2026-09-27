/// LeetCode #3964 - Minimum Lights to Illuminate a Road
fn min_lights(lights: Vec<i32>) -> i32 {
    let n = lights.len();
    let mut d = vec![0i32; n];
    for (i, v) in lights.into_iter().enumerate() {
        if v > 0 {
            let l = i.saturating_sub(v as usize);
            let r = (i + v as usize).min(n - 1);
            d[l] += 1;
            if r + 1 < n {
                d[r + 1] -= 1;
            }
        }
    }
    let mut s = 0;
    let mut cnt = 0;
    let mut ans = 0;
    for x in d {
        s += x;
        if s == 0 {
            cnt += 1;
        } else {
            ans += (cnt + 2) / 3;
            cnt = 0;
        }
    }
    ans + (cnt + 2) / 3
}

fn main() {
    println!("{}", min_lights(vec![0, 0, 0, 0]));
}

#[cfg(test)]
mod tests {
    use super::min_lights;

    #[test]
    fn example1() {
        assert_eq!(min_lights(vec![0, 0, 0, 0]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(min_lights(vec![0, 0, 0, 2, 0]), 1);
    }
}
