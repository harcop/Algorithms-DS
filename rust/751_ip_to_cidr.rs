/// LeetCode #751 - IP to CIDR
fn ip_to_cidr(ip: String, n: i32) -> Vec<String> {
    fn ip_to_u32(ip: &str) -> u32 {
        let p: Vec<u32> = ip.split('.').map(|x| x.parse().unwrap()).collect();
        (p[0] << 24) | (p[1] << 16) | (p[2] << 8) | p[3]
    }
    fn u32_to_ip(x: u32) -> String {
        format!(
            "{}.{}.{}.{}",
            (x >> 24) & 255,
            (x >> 16) & 255,
            (x >> 8) & 255,
            x & 255
        )
    }
    let mut x = ip_to_u32(&ip);
    let mut rem = n as u32;
    let mut out: Vec<String> = Vec::new();
    while rem > 0 {
        let mut step = (x as i64 & -(x as i64)) as u32;
        if step == 0 {
            step = 1 << 31;
        }
        while step > rem {
            step >>= 1;
        }
        let cidr = 32 - step.trailing_zeros() as i32;
        out.push(format!("{}/{}", u32_to_ip(x), cidr));
        x = x.wrapping_add(step);
        rem -= step;
    }
    out
}

fn main() {
    println!("{:?}", ip_to_cidr("255.0.0.7".into(), 10));
}

#[cfg(test)]
mod tests {
    use super::ip_to_cidr;

    #[test]
    fn example_one() {
        let mut v = ip_to_cidr("255.0.0.7".into(), 10);
        v.sort();
        let mut e: Vec<String> = vec![
            "255.0.0.7/32".into(),
            "255.0.0.8/29".into(),
            "255.0.0.16/32".into(),
        ];
        e.sort();
        assert_eq!(v, e);
    }
}
