use std::collections::HashMap;

use crate::address;

pub struct Store {
    w_addrs: HashMap<address::Deposit, Vec<address::Withdrawal>>,
    d_addrs: HashMap<Vec<address::Withdrawal>, address::Deposit>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            w_addrs: HashMap::new(),
            d_addrs: HashMap::new(),
        }
    }

    pub fn register(&mut self, w_addrs: &[address::Withdrawal]) -> address::Deposit {
        let d_addr = self
            .d_addrs
            .entry(w_addrs.into())
            .or_insert_with(address::Deposit::new);
        self.w_addrs.insert(d_addr.clone(), w_addrs.into());
        d_addr.clone()
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_is_idempotent() {
        let mut store = Store::new();
        let w_addrs = vec![
            address::Withdrawal::new("alice".into()),
            address::Withdrawal::new("bob".into()),
        ];
        assert_eq!(store.register(&w_addrs), store.register(&w_addrs));
    }
}
