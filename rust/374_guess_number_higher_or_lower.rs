/// LeetCode #374 - Guess Number Higher or Lower (binary search)
use std::cmp::Ordering;
use std::sync::Mutex;

static PICK: Mutex<i32> = Mutex::new(0);

fn guess(num: i32) -> i32 {
    let p = *PICK.lock().unwrap();
    match p.cmp(&num) {
        Ordering::Less => -1,
        Ordering::Greater => 1,
        Ordering::Equal => 0,
    }
}

fn guess_number(n: i32) -> i32 {
    let mut lo = 1;
    let mut hi = n;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match guess(mid) {
            0 => return mid,
            1 => lo = mid + 1,
            _ => hi = mid - 1,
        }
    }
    lo
}

fn main() {
    *PICK.lock().unwrap() = 6;
    println!("{}", guess_number(10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs() {
        *PICK.lock().unwrap() = 5;
        assert_eq!(guess_number(10), 5);
        *PICK.lock().unwrap() = 1;
        assert_eq!(guess_number(1), 1);
    }
}
