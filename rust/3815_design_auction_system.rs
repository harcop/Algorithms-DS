/// LeetCode #3815 - Design Auction System
use std::collections::{BTreeSet, HashMap};

struct AuctionSystem {
    items: HashMap<i32, BTreeSet<(i32, i32)>>,
    users: HashMap<i32, HashMap<i32, i32>>,
}

impl AuctionSystem {
    fn new() -> Self {
        Self {
            items: HashMap::new(),
            users: HashMap::new(),
        }
    }

    fn add_bid(&mut self, user_id: i32, item_id: i32, bid_amount: i32) {
        if self
            .users
            .get(&user_id)
            .map(|m| m.contains_key(&item_id))
            .unwrap_or(false)
        {
            self.remove_bid(user_id, item_id);
        }
        self.users
            .entry(user_id)
            .or_default()
            .insert(item_id, bid_amount);
        self.items
            .entry(item_id)
            .or_default()
            .insert((bid_amount, user_id));
    }

    fn update_bid(&mut self, user_id: i32, item_id: i32, new_amount: i32) {
        let old_amount = self.users[&user_id][&item_id];
        if let Some(set) = self.items.get_mut(&item_id) {
            set.remove(&(old_amount, user_id));
            set.insert((new_amount, user_id));
        }
        self.users
            .entry(user_id)
            .or_default()
            .insert(item_id, new_amount);
    }

    fn remove_bid(&mut self, user_id: i32, item_id: i32) {
        let old_amount = self.users[&user_id][&item_id];
        if let Some(set) = self.items.get_mut(&item_id) {
            set.remove(&(old_amount, user_id));
        }
        if let Some(m) = self.users.get_mut(&user_id) {
            m.remove(&item_id);
        }
    }

    fn get_highest_bidder(&self, item_id: i32) -> i32 {
        match self.items.get(&item_id).and_then(|s| s.last()) {
            Some(&(_, uid)) => uid,
            None => -1,
        }
    }
}

fn main() {
    let mut a = AuctionSystem::new();
    a.add_bid(1, 7, 5);
    println!("{}", a.get_highest_bidder(7));
}

#[cfg(test)]
mod tests {
    use super::AuctionSystem;

    #[test]
    fn example1() {
        let mut a = AuctionSystem::new();
        a.add_bid(1, 7, 5);
        a.add_bid(2, 7, 6);
        assert_eq!(a.get_highest_bidder(7), 2);
        a.update_bid(1, 7, 8);
        assert_eq!(a.get_highest_bidder(7), 1);
        a.remove_bid(2, 7);
        assert_eq!(a.get_highest_bidder(7), 1);
        assert_eq!(a.get_highest_bidder(3), -1);
    }
}
