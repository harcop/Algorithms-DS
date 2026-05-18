/// LeetCode #1061 - Lexicographically Smallest Equivalent String
struct Dsu {
    parent: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Dsu { parent: (0..n).collect() }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return;
        }
        if ra < rb {
            self.parent[rb] = ra;
        } else {
            self.parent[ra] = rb;
        }
    }
}

fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
    let mut dsu = Dsu::new(26);
    for (a, b) in s1.bytes().zip(s2.bytes()) {
        dsu.union((a - b'a') as usize, (b - b'a') as usize);
    }
    base_str
        .bytes()
        .map(|c| (b'a' + dsu.find((c - b'a') as usize) as u8) as char)
        .collect()
}

fn main() {
    println!("{}", smallest_equivalent_string("parker".into(), "morris".into(), "parser".into()));
}

#[cfg(test)]
mod tests {
    use super::smallest_equivalent_string;

    #[test]
    fn example_one() {
        assert_eq!(
            smallest_equivalent_string("parker".into(), "morris".into(), "parser".into()),
            "makkek"
        );
    }
}
