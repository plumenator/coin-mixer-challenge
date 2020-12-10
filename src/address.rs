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

#[derive(PartialEq, Eq, Debug)]
pub struct Deposit(String);

const MIN_ADDRESS_LEN: usize = 26;
const MAX_ADDRESS_LEN: usize = 35;

impl Deposit {
    pub fn new() -> Self {
        use rand::distributions::Alphanumeric;
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        let addr_str: String = rng
            .sample_iter(&Alphanumeric)
            .take(rng.gen_range(MIN_ADDRESS_LEN, MAX_ADDRESS_LEN + 1))
            .collect();

        Self(addr_str)
    }
}

impl Default for Deposit {
    fn default() -> Self {
        Self::new()
    }
}

impl ToString for Deposit {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_is_random() {
        assert_ne!(Deposit::new(), Deposit::default());
    }

    #[test]
    fn address_is_in_range() {
        let addr = Deposit::default();
        assert!(addr.0.len() >= MIN_ADDRESS_LEN);
        assert!(addr.0.len() <= MAX_ADDRESS_LEN);
    }
}
