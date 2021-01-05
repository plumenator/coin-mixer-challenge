use std::time::Duration;

use bigdecimal::{BigDecimal, Zero};

use crate::{address, api::Api, store::Store};

pub struct Mixer {
    house_addr: address::House,
}

impl Mixer {
    pub fn new(api: &Api) -> anyhow::Result<Self> {
        Ok(Self {
            house_addr: address::Unused::new(&api)?.into(),
        })
    }

    pub fn run(&self, api: &Api, store: &Store) -> anyhow::Result<()> {
        let all_deposits = store.all_deposits();
        loop {
            let non_empty = all_deposits.iter().filter_map(|d| {
                let deposit_info = api.address_info(d.as_str()).ok()?;
                if !deposit_info.balance.is_zero() {
                    Some((d, deposit_info))
                } else {
                    None
                }
            });
            for (deposit_addr, deposit_info) in non_empty {
                println!("Detected deposit of amount: {}", deposit_info.balance);
                api.send_all(deposit_addr.as_str(), self.house_addr.as_str())?;
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
                    )?;
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
    for _ in 0..9 {
        let duration = Duration::from_millis(rng.gen_range(1, 101));
        let address = w_addrs.choose(&mut rng).expect("an address").clone();
        let fraction: f64 = rng.gen_range(0f64, 1f64);
        let partial_amount = BigDecimal::try_from(fraction).expect("a bigdecimal") * &amount;
        parts.push((duration, address, partial_amount.clone()));
        amount -= partial_amount;
    }
    let duration = Duration::from_millis(rng.gen_range(1, 101));
    let address = w_addrs.choose(&mut rng).expect("an address").clone();
    parts.push((duration, address, amount));
    parts
}
