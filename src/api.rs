use anyhow::{anyhow, Context};
use bigdecimal::{BigDecimal, Zero};
use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
pub(crate) struct AddressInfo {
    pub(crate) balance: BigDecimal,
    transactions: Vec<TransactionInfo>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TransactionInfo {
    _timestamp: String,
    _to_address: String,
    _from_address: Option<String>,
    _amount: BigDecimal,
}

pub struct Api {
    addresses_url: Url,
    transactions_url: Url,
}

impl Api {
    pub fn new(mut base_url: Url) -> anyhow::Result<Self> {
        // the trailing slash is significant
        base_url
            .path_segments_mut()
            .map_err(|_| anyhow!("given URL cannot be a base"))?
            .push("/");
        // the trailing slash is significant
        let addresses_url = base_url.join("addresses/")?;
        // the trailing slash is unnecessary, because we don't use this as a base url
        let transactions_url = base_url.join("transactions")?;
        Ok(Self {
            addresses_url,
            transactions_url,
        })
    }

    pub(crate) fn is_unused_address(&self, addr_str: &str) -> anyhow::Result<bool> {
        let AddressInfo {
            balance,
            transactions,
        } = self.address_info(addr_str)?;
        Ok(balance.is_zero() && transactions.is_empty())
    }

    pub(crate) fn address_info(&self, addr_str: &str) -> anyhow::Result<AddressInfo> {
        reqwest::blocking::get(self.addresses_url.join(addr_str).expect("/{address}"))
            .context("could not GET address info")?
            .json()
            .context("could not parse response as JSON")
    }

    pub(crate) fn send_all(&self, from_addr: &str, to_addr: &str) -> anyhow::Result<()> {
        let AddressInfo { balance, .. } = self.address_info(from_addr)?;
        self.send_part(from_addr, to_addr, balance)
    }

    pub(crate) fn send_part(
        &self,
        from_addr: &str,
        to_addr: &str,
        amount: BigDecimal,
    ) -> anyhow::Result<()> {
        let amount = amount.to_string();
        let params = [
            ("fromAddress", from_addr),
            ("toAddress", to_addr),
            ("amount", &amount),
        ];

        let client = reqwest::blocking::Client::new();
        // TODO: handle the insufficient balance error
        client
            .post(self.transactions_url.clone())
            .query(&params)
            .send()
            .context("could not POST send request")
            .map(|_| ())
    }
}
