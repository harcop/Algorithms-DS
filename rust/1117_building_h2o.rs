/// LeetCode #1117 - Building H2O
use std::sync::atomic::{AtomicI32, Ordering};

pub struct H2O {
    h: AtomicI32,
}

impl H2O {
    pub fn new() -> Self {
        Self {
            h: AtomicI32::new(0),
        }
    }

    pub fn hydrogen(&self, release_hydrogen: impl Fn()) {
        loop {
            let v = self.h.load(Ordering::SeqCst);
            if v >= 0 && v < 2 {
                if self
                    .h
                    .compare_exchange(v, v + 1, Ordering::SeqCst, Ordering::SeqCst)
                    .is_ok()
                {
                    release_hydrogen();
                    break;
                }
            } else {
                std::hint::spin_loop();
            }
        }
    }

    pub fn oxygen(&self, release_oxygen: impl Fn()) {
        loop {
            let v = self.h.load(Ordering::SeqCst);
            if v == 2 {
                if self
                    .h
                    .compare_exchange(2, 0, Ordering::SeqCst, Ordering::SeqCst)
                    .is_ok()
                {
                    release_oxygen();
                    break;
                }
            } else {
                std::hint::spin_loop();
            }
        }
    }
}

fn main() {
    let h2o = H2O::new();
    h2o.hydrogen(|| print!("H"));
    h2o.hydrogen(|| print!("H"));
    h2o.oxygen(|| print!("O"));
}

#[cfg(test)]
mod tests {
    use super::H2O;

    #[test]
    fn smoke() {
        let h2o = H2O::new();
        h2o.hydrogen(|| {});
        h2o.hydrogen(|| {});
        h2o.oxygen(|| {});
        assert_eq!(h2o.h.load(std::sync::atomic::Ordering::SeqCst), 0);
    }
}
