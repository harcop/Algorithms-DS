/// LeetCode #1114 - Print in Order
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Foo {
    stage: AtomicUsize,
}

impl Foo {
    pub fn new() -> Self {
        Self {
            stage: AtomicUsize::new(0),
        }
    }

    pub fn first(&self, print_first: impl FnOnce()) {
        print_first();
        self.stage.store(1, Ordering::SeqCst);
    }

    pub fn second(&self, print_second: impl FnOnce()) {
        while self.stage.load(Ordering::SeqCst) < 1 {
            std::hint::spin_loop();
        }
        print_second();
        self.stage.store(2, Ordering::SeqCst);
    }

    pub fn third(&self, print_third: impl FnOnce()) {
        while self.stage.load(Ordering::SeqCst) < 2 {
            std::hint::spin_loop();
        }
        print_third();
    }
}

fn main() {
    let foo = Foo::new();
    foo.first(|| print!("first"));
    foo.second(|| print!("second"));
    foo.third(|| print!("third"));
}

#[cfg(test)]
mod tests {
    use super::Foo;
    use std::sync::Mutex;

    #[test]
    fn order() {
        let foo = Foo::new();
        let log = Mutex::new(String::new());
        foo.first(|| log.lock().unwrap().push_str("1"));
        foo.second(|| log.lock().unwrap().push_str("2"));
        foo.third(|| log.lock().unwrap().push_str("3"));
        assert_eq!(*log.lock().unwrap(), "123");
    }
}
