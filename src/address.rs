use crate::api::Api;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct Withdrawal(String);

impl Withdrawal {
    pub fn new(addr_str: String) -> Self {
        Self(addr_str)
    }
}

impl ToString for Withdrawal {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct Deposit(String);

impl Deposit {
    pub(crate) fn new(unused: Unused) -> Self {
        unused.into()
    }
}

impl ToString for Deposit {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl From<Unused> for Deposit {
    fn from(unused: Unused) -> Self {
        Self(unused.0)
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub(crate) struct Unused(String);

impl Unused {
    pub(crate) fn new(api: &Api) -> Self {
        loop {
            let addr_str = generate_address_string();
            if api.is_unused_address(&addr_str) {
                return Self(addr_str);
            }
        }
    }
}

impl ToString for Unused {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

const MIN_ADDRESS_LEN: usize = 26;
const MAX_ADDRESS_LEN: usize = 35;

fn generate_address_string() -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();
    rng.sample_iter(&Alphanumeric)
        .take(rng.gen_range(MIN_ADDRESS_LEN, MAX_ADDRESS_LEN + 1))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_is_random() {
        assert_ne!(generate_address_string(), generate_address_string());
    }

    #[test]
    fn address_is_in_range() {
        let addr = generate_address_string();
        assert!(addr.len() >= MIN_ADDRESS_LEN);
        assert!(addr.len() <= MAX_ADDRESS_LEN);
    }
}
