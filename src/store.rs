use std::collections::HashMap;

use anyhow::ensure;

use crate::{address, api::Api};

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

    pub async fn register(
        &mut self,
        api: &Api,
        w_addrs: &[address::Withdrawal],
    ) -> anyhow::Result<address::Deposit> {
        ensure!(!w_addrs.is_empty());
        let unused_addr = address::Unused::new(api).await?;
        let d_addr = self
            .d_addrs
            .entry(w_addrs.into())
            .or_insert_with(|| address::Deposit::new(unused_addr));
        self.w_addrs.insert(d_addr.clone(), w_addrs.into());
        Ok(d_addr.clone())
    }

    pub(crate) fn all_deposits(&self) -> Vec<address::Deposit> {
        self.w_addrs.keys().cloned().collect()
    }

    pub(crate) fn all_withdrawals(
        &self,
        deposit_addr: &address::Deposit,
    ) -> Vec<address::Withdrawal> {
        self.w_addrs
            .get(deposit_addr)
            .expect("gets withdrawal addresses")
            .clone()
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

    #[tokio::test]
    async fn register_is_idempotent() {
        let mut store = Store::new();
        let w_addrs = vec![
            address::Withdrawal::new("alice".into()),
            address::Withdrawal::new("bob".into()),
        ];
        let api = Api::new(
            url::Url::parse("https://jobcoin.gemini.com/marmalade-manual/api").expect("parse"),
        )
        .expect("api");
        assert_eq!(
            store.register(&api, &w_addrs).await.expect("register once"),
            store
                .register(&api, &w_addrs)
                .await
                .expect("register again")
        );
    }
}
