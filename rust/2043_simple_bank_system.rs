/// LeetCode #2043 - Simple Bank System
pub struct Bank {
    balance: Vec<i64>,
    n: usize,
}

impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        let n = balance.len();
        Bank { balance, n }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        let a1 = account1 as usize;
        let a2 = account2 as usize;
        if a1 == 0 || a2 == 0 || a1 > self.n || a2 > self.n || self.balance[a1 - 1] < money {
            return false;
        }
        self.balance[a1 - 1] -= money;
        self.balance[a2 - 1] += money;
        true
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        let a = account as usize;
        if a == 0 || a > self.n {
            return false;
        }
        self.balance[a - 1] += money;
        true
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        let a = account as usize;
        if a == 0 || a > self.n || self.balance[a - 1] < money {
            return false;
        }
        self.balance[a - 1] -= money;
        true
    }
}

fn main() {
    let mut bank = Bank::new(vec![10, 100, 20, 50, 30]);
    println!("{}", bank.withdraw(3, 10));
}

#[cfg(test)]
mod tests {
    use super::Bank;

    #[test]
    fn example() {
        let mut bank = Bank::new(vec![10, 100, 20, 50, 30]);
        assert!(bank.withdraw(3, 10));
        assert!(bank.transfer(5, 1, 20));
        assert!(bank.deposit(5, 20));
        assert!(!bank.transfer(3, 4, 15));
        assert!(!bank.withdraw(10, 50));
    }
}
