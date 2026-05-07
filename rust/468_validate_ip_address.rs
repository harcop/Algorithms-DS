/// LeetCode #468 - Validate IP Address
fn valid_ip_address(query_ip: String) -> String {
    fn is_ipv4(s: &str) -> bool {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 4 {
            return false;
        }
        for p in parts {
            if p.is_empty() || p.len() > 3 {
                return false;
            }
            if p.len() > 1 && p.starts_with('0') {
                return false;
            }
            if !p.chars().all(|c| c.is_ascii_digit()) {
                return false;
            }
            if p.parse::<i32>().unwrap_or(-1) > 255 {
                return false;
            }
        }
        true
    }

    fn is_hex(b: u8) -> bool {
        matches!(b, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')
    }

    fn is_ipv6(s: &str) -> bool {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 8 {
            return false;
        }
        for p in parts {
            if p.is_empty() || p.len() > 4 {
                return false;
            }
            if !p.bytes().all(is_hex) {
                return false;
            }
        }
        true
    }

    if is_ipv4(&query_ip) {
        "IPv4".into()
    } else if is_ipv6(&query_ip) {
        "IPv6".into()
    } else {
        "Neither".into()
    }
}

fn main() {
    println!("{}", valid_ip_address("172.16.254.1".into()));
}

#[cfg(test)]
mod tests {
    use super::valid_ip_address;

    #[test]
    fn example_one() {
        assert_eq!(valid_ip_address("172.16.254.1".into()), "IPv4");
    }

    #[test]
    fn example_two() {
        assert_eq!(
            valid_ip_address("2001:0db8:85a3:0:0:8A2E:0370:7334".into()),
            "IPv6"
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(valid_ip_address("256.256.256.256".into()), "Neither");
    }
}
