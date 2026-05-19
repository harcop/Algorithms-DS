/// LeetCode #1108 - Defang an IP Address
fn defang_i_paddr(address: String) -> String {
    address.replace('.', "[.]")
}

fn main() {
    println!("{}", defang_i_paddr("255.255.11.135".into()));
}

#[cfg(test)]
mod tests {
    use super::defang_i_paddr;

    #[test]
    fn example_one() {
        assert_eq!(defang_i_paddr("1.1.1.1".into()), "1[.]1[.]1[.]1");
    }
}
