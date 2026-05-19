/// LeetCode #1104 - Defanging an IP Address
fn defang_i_paddr(address: String) -> String {
    address.replace('.', "[.]")
}

fn main() {
    println!("{}", defang_i_paddr("1.1.1.1".into()));
}

#[cfg(test)]
mod tests {
    use super::defang_i_paddr;

    #[test]
    fn example_one() {
        assert_eq!(defang_i_paddr("1.1.1.1".into()), "1[.]1[.]1[.]1");
    }

    #[test]
    fn example_two() {
        assert_eq!(defang_i_paddr("8.8.8.8".into()), "8[.]8[.]8[.]8");
    }
}
