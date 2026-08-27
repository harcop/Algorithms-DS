/// LeetCode #3451 - Find Invalid IP Addresses (SQL; Rust analogue)
/// logs: (log_id, ip, status_code)
fn is_valid_ip(ip: &str) -> bool {
    let octets: Vec<&str> = ip.split('.').collect();
    if octets.len() != 4 {
        return false;
    }
    for octet in octets {
        if octet.is_empty() || !octet.bytes().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let Ok(value) = octet.parse::<i32>() else {
            return false;
        };
        if !(0..=255).contains(&value) || octet != value.to_string() {
            return false;
        }
    }
    true
}

fn find_invalid_ips(logs: Vec<(i32, String, i32)>) -> Vec<(String, i32)> {
    use std::collections::HashMap;
    let mut cnt: HashMap<String, i32> = HashMap::new();
    for (_, ip, _) in logs {
        if !is_valid_ip(&ip) {
            *cnt.entry(ip).or_insert(0) += 1;
        }
    }
    let mut ans: Vec<(String, i32)> = cnt.into_iter().collect();
    ans.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));
    ans
}

fn main() {
    let logs = vec![
        (1, "192.168.1.1".into(), 200),
        (2, "256.1.2.3".into(), 404),
        (3, "192.168.001.1".into(), 200),
        (4, "192.168.1.1".into(), 200),
        (5, "192.168.1".into(), 500),
        (6, "256.1.2.3".into(), 404),
        (7, "192.168.001.1".into(), 200),
    ];
    println!("{:?}", find_invalid_ips(logs));
}

#[cfg(test)]
mod tests {
    use super::find_invalid_ips;

    #[test]
    fn example() {
        let logs = vec![
            (1, "192.168.1.1".into(), 200),
            (2, "256.1.2.3".into(), 404),
            (3, "192.168.001.1".into(), 200),
            (4, "192.168.1.1".into(), 200),
            (5, "192.168.1".into(), 500),
            (6, "256.1.2.3".into(), 404),
            (7, "192.168.001.1".into(), 200),
        ];
        assert_eq!(
            find_invalid_ips(logs),
            vec![
                ("256.1.2.3".into(), 2),
                ("192.168.001.1".into(), 2),
                ("192.168.1".into(), 1),
            ]
        );
    }
}
