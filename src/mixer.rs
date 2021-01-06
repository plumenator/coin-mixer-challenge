use std::time::Duration;

use bigdecimal::{BigDecimal, Zero};

use crate::{address, api::Api, store::Store};

pub struct Mixer {
    house_addr: address::House,
}

impl Mixer {
    pub async fn new(api: &Api) -> anyhow::Result<Self> {
        Ok(Self {
            house_addr: address::Unused::new(&api).await?.into(),
        })
    }

    pub async fn run(&self, api: &Api, store: &Store) -> anyhow::Result<()> {
        let all_deposits = store.all_deposits();
        loop {
            for deposit_addr in &all_deposits {
                let deposit_info = api.address_info(deposit_addr.as_str()).await?;
                if deposit_info.balance.is_zero() {
                    continue;
                }
                println!("Detected deposit of amount: {}", deposit_info.balance);
                api.send_all(deposit_addr.as_str(), self.house_addr.as_str())
                    .await?;
                println!("Sending to house address: {}", self.house_addr.to_string());
                let w_addrs = store.all_withdrawals(&deposit_addr);
                for (wait_duration, w_addr, partial_amount) in
                    randomized(w_addrs, deposit_info.balance)
                {
                    println!("Waiting for {} ms", wait_duration.as_millis());
                    std::thread::sleep(wait_duration);
                    api.send_part(
                        self.house_addr.as_str(),
                        w_addr.as_str(),
                        partial_amount.clone(),
                    )
                    .await?;
                    println!("Sending {} to {}", partial_amount, w_addr.as_str());
                }
                println!("Done!");
            }
            std::thread::sleep(Duration::from_secs(1));
        }
    }
}

fn randomized(
    w_addrs: Vec<address::Withdrawal>,
    mut amount: BigDecimal,
) -> Vec<(Duration, address::Withdrawal, BigDecimal)> {
    use std::convert::TryFrom;

    use rand::{prelude::SliceRandom, thread_rng, Rng};

    let mut rng = thread_rng();
    let mut parts = Vec::new();
    let fractions = gen_fractions(&mut rng, 10);
    for fraction in &fractions[1..] {
        let duration = Duration::from_millis(rng.gen_range(1, 101));
        let address = w_addrs.choose(&mut rng).expect("an address").clone();
        let partial_amount = BigDecimal::try_from(*fraction).expect("a bigdecimal") * &amount;
        parts.push((duration, address, partial_amount.clone()));
        amount -= partial_amount;
    }
    let duration = Duration::from_millis(rng.gen_range(1, 101));
    let address = w_addrs.choose(&mut rng).expect("an address").clone();
    parts.push((duration, address, amount));
    parts
}

fn gen_fractions(rng: &mut impl rand::Rng, size: usize) -> Vec<f64> {
    let mut numerators = vec![0u8; size];
    rng.fill(&mut numerators[..]);
    let denominator: u16 = numerators.iter().map(|x| *x as u16).sum();
    numerators
        .iter()
        .map(|x| *x as f64 / denominator as f64)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_parts_decay() -> bool {
        let parts = randomized(
            vec![
                address::Withdrawal::new("add1".into()),
                address::Withdrawal::new("add2".into()),
                address::Withdrawal::new("add3".into()),
            ],
            10.into(),
        );
        parts[0].2 > parts[parts.len() - 1].2
    }

    #[test]
    fn parts_dont_decay() {
        let results = vec![
            do_parts_decay(),
            do_parts_decay(),
            do_parts_decay(),
            do_parts_decay(),
            do_parts_decay(),
        ];

        assert!(!results.iter().all(|x| *x))
    }
}
