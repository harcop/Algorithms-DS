/// LeetCode #157 - Read N Characters Given Read4
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

    /// Reads up to 4 bytes from the simulated file into `buf4`.
    pub fn read4(&mut self, buf4: &mut [u8; 4]) -> i32 {
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
    let mut r = Read4Reader::new("abcdef");
    let mut buf = [0u8; 10];
    let k = r.read(&mut buf, 5);
    println!("{}", k);
}

#[cfg(test)]
mod tests {
    use super::Read4Reader;

    #[test]
    fn read_across_chunks() {
        let mut r = Read4Reader::new("abcdef");
        let mut buf = [0u8; 10];
        assert_eq!(r.read(&mut buf, 5), 5);
        assert_eq!(&buf[..5], b"abcde");
    }

    #[test]
    fn read_partial_last_chunk() {
        let mut r = Read4Reader::new("abcd");
        let mut buf = [0u8; 10];
        assert_eq!(r.read(&mut buf, 3), 3);
        assert_eq!(&buf[..3], b"abc");
    }

    #[test]
    fn multiple_reads_preserve_file() {
        let mut r = Read4Reader::new("abcdefgh");
        let mut buf = [0u8; 10];
        assert_eq!(r.read(&mut buf, 3), 3);
        assert_eq!(&buf[..3], b"abc");
        assert_eq!(r.read(&mut buf, 4), 4);
        assert_eq!(&buf[..4], b"defg");
    }
}
