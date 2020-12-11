use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
struct AddressInfo {
    balance: String,
    transactions: Vec<TransactionInfo>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TransactionInfo {
    timestamp: String,
    to_address: String,
    from_address: Option<String>,
    amount: String,
}

pub struct Api {
    addresses_url: Url,
}

impl Api {
    pub fn new(mut base_url: Url) -> Self {
        // the trailing slash is significant
        base_url
            .path_segments_mut()
            .expect("path segments")
            .push("/");
        let addresses_url = base_url.join("addresses/").expect("/addresses");
        Self { addresses_url }
    }

    pub(crate) fn is_unused_address(&self, addr_str: &str) -> bool {
        let AddressInfo {
            balance,
            transactions,
        } = reqwest::blocking::get(self.addresses_url.join(addr_str).expect("/{address}"))
            .expect("gets address info")
            .json()
            .expect("parses json response");
        balance == "0" && transactions.is_empty()
    }
}
