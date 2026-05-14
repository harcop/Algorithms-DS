/// LeetCode #721 - Accounts Merge
use std::collections::HashMap;

struct DSU {
    p: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self { p: (0..n).collect() }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.p[x] != x {
            self.p[x] = self.find(self.p[x]);
        }
        self.p[x]
    }
    fn union(&mut self, a: usize, b: usize) {
        let pa = self.find(a);
        let pb = self.find(b);
        if pa != pb {
            self.p[pa] = pb;
        }
    }
}

fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let n = accounts.len();
    let mut uf = DSU::new(n);
    let mut email_owner: HashMap<String, usize> = HashMap::new();
    for (i, acct) in accounts.iter().enumerate() {
        for e in acct.iter().skip(1) {
            if let Some(&j) = email_owner.get(e) {
                uf.union(i, j);
            } else {
                email_owner.insert(e.clone(), i);
            }
        }
    }
    let mut groups: HashMap<usize, Vec<String>> = HashMap::new();
    for (email, &acc_idx) in &email_owner {
        let r = uf.find(acc_idx);
        groups.entry(r).or_default().push(email.clone());
    }
    let mut out: Vec<Vec<String>> = Vec::new();
    for (root, mut emails) in groups {
        emails.sort();
        let name = accounts[root][0].clone();
        let mut row = vec![name];
        row.extend(emails);
        out.push(row);
    }
    out.sort_by(|a, b| a[0].cmp(&b[0]));
    out
}

fn main() {
    let a = vec![
        vec![
            "John".into(),
            "johnsmith@mail.com".into(),
            "john_newyork@mail.com".into(),
        ],
        vec![
            "John".into(),
            "johnsmith@mail.com".into(),
            "john00@mail.com".into(),
        ],
        vec![
            "Mary".into(),
            "mary@mail.com".into(),
        ],
        vec!["John".into(), "johnnybravo@mail.com".into()],
    ];
    println!("{:?}", accounts_merge(a));
}

#[cfg(test)]
mod tests {
    use super::accounts_merge;

    fn normalize(mut v: Vec<Vec<String>>) -> Vec<Vec<String>> {
        for row in v.iter_mut() {
            if row.len() > 1 {
                row[1..].sort();
            }
        }
        v.sort_by(|a, b| a.join("\n").cmp(&b.join("\n")));
        v
    }

    #[test]
    fn example_one() {
        let a = vec![
            vec![
                "John".into(),
                "johnsmith@mail.com".into(),
                "john_newyork@mail.com".into(),
            ],
            vec![
                "John".into(),
                "johnsmith@mail.com".into(),
                "john00@mail.com".into(),
            ],
            vec![
                "Mary".into(),
                "mary@mail.com".into(),
            ],
            vec!["John".into(), "johnnybravo@mail.com".into()],
        ];
        let got = normalize(accounts_merge(a));
        let exp = normalize(vec![
            vec![
                "John".into(),
                "john00@mail.com".into(),
                "john_newyork@mail.com".into(),
                "johnsmith@mail.com".into(),
            ],
            vec!["Mary".into(), "mary@mail.com".into()],
            vec!["John".into(), "johnnybravo@mail.com".into()],
        ]);
        assert_eq!(got, exp);
    }
}
