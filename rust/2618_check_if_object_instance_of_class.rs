/// LeetCode #2618 - Check if Object Instance of Class (JS problem; Rust TypeId analogue)
use std::any::{Any, TypeId};

fn check_if_instance_of<T: Any + ?Sized>(obj: Option<&T>, class_type_id: TypeId) -> bool {
    match obj {
        Some(o) => o.type_id() == class_type_id,
        None => false,
    }
}

fn main() {
    let x = 5i32;
    println!(
        "{}",
        check_if_instance_of(Some(&x), TypeId::of::<i32>())
    );
}

#[cfg(test)]
mod tests {
    use super::check_if_instance_of;
    use std::any::TypeId;

    #[test]
    fn same_type() {
        let d = String::from("hi");
        assert!(check_if_instance_of(Some(&d), TypeId::of::<String>()));
    }

    #[test]
    fn different_type() {
        let d = String::from("hi");
        assert!(!check_if_instance_of(Some(&d), TypeId::of::<i32>()));
    }

    #[test]
    fn none_is_false() {
        let x: Option<&i32> = None;
        assert!(!check_if_instance_of(x, TypeId::of::<i32>()));
    }
}
