use std::io::{self, BufRead};

use anyhow::Context;
use structopt::StructOpt;

use coin_mixer_challenge::{address, api::Api, mixer::Mixer, store::Store};

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(long)]
    api_base_url: url::Url,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Options { api_base_url } = Options::from_args();
    println!("Given API Base URL:");
    println!("{}", api_base_url);
    let api = Api::new(api_base_url)?;
    let mut w_addrs = Vec::new();
    for addr_str in io::stdin().lock().lines() {
        w_addrs.push(address::Withdrawal::new(
            addr_str.context("unable to read from stdin")?,
        ));
    }
    println!("Read the following withdrawal addresses:");
    for w_addr in &w_addrs {
        println!("{}", w_addr.to_string());
    }
    let mut store = Store::new();
    let d_addr = store.register(&api, &w_addrs).await?;
    println!("Generated deposit address:");
    println!("{}", d_addr.to_string());
    let mixer = Mixer::new(&api).await?;
    mixer.run(&api, &store).await
}
