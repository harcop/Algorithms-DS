/// LeetCode #278 - First Bad Version
pub static mut BAD: i32 = 1;

fn is_bad_version(v: i32) -> bool {
    unsafe { v >= BAD }
}

fn first_bad_version(n: i32) -> i32 {
    let mut lo = 1i32;
    let mut hi = n;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if is_bad_version(mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

fn main() {
    unsafe { BAD = 4; }
    println!("{}", first_bad_version(5));
}

#[cfg(test)]
mod tests {
    use super::{first_bad_version, BAD};

    #[test]
    fn example_one() {
        unsafe {
            BAD = 4;
        }
        assert_eq!(first_bad_version(5), 4);
    }

    #[test]
    fn example_two() {
        unsafe {
            BAD = 1;
        }
        assert_eq!(first_bad_version(1), 1);
    }
}
