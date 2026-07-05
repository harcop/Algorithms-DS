/// LeetCode #2241 - Design an ATM Machine
struct Atm {
    denom: [i32; 5],
    cnt: [i64; 5],
}

impl Atm {
    fn new() -> Self {
        Atm {
            denom: [20, 50, 100, 200, 500],
            cnt: [0; 5],
        }
    }

    fn deposit(&mut self, banknotes_count: Vec<i32>) {
        for (i, &x) in banknotes_count.iter().enumerate() {
            self.cnt[i] += x as i64;
        }
    }

    fn withdraw(&mut self, amount: i32) -> Vec<i32> {
        let mut amount = amount as i64;
        let mut ans = [0i64; 5];
        for i in (0..5).rev() {
            ans[i] = (amount / self.denom[i] as i64).min(self.cnt[i]);
            amount -= ans[i] * self.denom[i] as i64;
        }
        if amount > 0 {
            return vec![-1];
        }
        for i in 0..5 {
            self.cnt[i] -= ans[i];
        }
        ans.iter().map(|&x| x as i32).collect()
    }
}

fn main() {
    let mut atm = Atm::new();
    atm.deposit(vec![0, 0, 1, 2, 1]);
    println!("{:?}", atm.withdraw(600));
}

#[cfg(test)]
mod tests {
    use super::Atm;

    #[test]
    fn example() {
        let mut atm = Atm::new();
        atm.deposit(vec![0, 0, 1, 2, 1]);
        assert_eq!(atm.withdraw(600), vec![0, 0, 1, 0, 1]);
        atm.deposit(vec![0, 1, 0, 1, 1]);
        assert_eq!(atm.withdraw(600), vec![-1]);
        assert_eq!(atm.withdraw(550), vec![0, 1, 0, 0, 1]);
    }
}
