/// LeetCode #3204 - Bitwise User Permissions Analysis (SQL; Rust analogue)
/// permissions: list of permission bitmasks
fn bitwise_user_permissions(permissions: Vec<i32>) -> (i32, i32) {
    let common = permissions.iter().copied().reduce(|a, b| a & b).unwrap_or(0);
    let any = permissions.iter().copied().reduce(|a, b| a | b).unwrap_or(0);
    (common, any)
}

fn main() {
    println!("{:?}", bitwise_user_permissions(vec![5, 12, 7, 3]));
}

#[cfg(test)]
mod tests {
    use super::bitwise_user_permissions;

    #[test]
    fn example() {
        assert_eq!(bitwise_user_permissions(vec![5, 12, 7, 3]), (0, 15));
    }
}
