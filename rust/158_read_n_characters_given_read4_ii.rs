/// LeetCode #158 - Read N Characters Given Read4 II - Call multiple times
pub struct Read4Reader {
    data: Vec<u8>,
    pos: usize,
    stash: Vec<u8>,
    stash_i: usize,
}

impl Read4Reader {
    pub fn new(data: impl Into<Vec<u8>>) -> Self {
        Read4Reader {
            data: data.into(),
            pos: 0,
            stash: Vec::new(),
            stash_i: 0,
        }
    }

    fn read4(&mut self, buf4: &mut [u8; 4]) -> i32 {
        let mut w = 0usize;
        while w < 4 && self.pos < self.data.len() {
            buf4[w] = self.data[self.pos];
            self.pos += 1;
            w += 1;
        }
        w as i32
    }

    fn compact_stash(&mut self) {
        if self.stash_i > 0 && self.stash_i <= self.stash.len() {
            self.stash.drain(0..self.stash_i);
            self.stash_i = 0;
        }
    }

    pub fn read(&mut self, buf: &mut [u8], n: i32) -> i32 {
        let n = n as usize;
        let mut written = 0usize;
        let mut tmp = [0u8; 4];

        while written < n {
            while written < n && self.stash_i < self.stash.len() {
                buf[written] = self.stash[self.stash_i];
                self.stash_i += 1;
                written += 1;
            }
            self.compact_stash();

            if written >= n {
                break;
            }

            let got = self.read4(&mut tmp) as usize;
            if got == 0 {
                break;
            }
            let need = n - written;
            let take = got.min(need);
            buf[written..written + take].copy_from_slice(&tmp[..take]);
            written += take;
            if take < got {
                self.stash.extend_from_slice(&tmp[take..got]);
            }
        }
        written as i32
    }
}

fn main() {
    let mut r = Read4Reader::new("abc");
    let mut b1 = [0u8; 4];
    let mut b2 = [0u8; 4];
    println!("{} {}", r.read(&mut b1, 1), r.read(&mut b2, 2));
}

#[cfg(test)]
mod tests {
    use super::Read4Reader;

    #[test]
    fn staggered_reads() {
        let mut r = Read4Reader::new("abcdef");
        let mut a = [0u8; 8];
        let mut b = [0u8; 8];
        assert_eq!(r.read(&mut a, 1), 1);
        assert_eq!(r.read(&mut b, 4), 4);
        assert_eq!(&a[..1], b"a");
        assert_eq!(&b[..4], b"bcde");
        assert_eq!(r.read(&mut a, 10), 1);
        assert_eq!(&a[..1], b"f");
    }
}
