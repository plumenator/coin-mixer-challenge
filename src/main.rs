use std::io::{self, BufRead};

use structopt::StructOpt;

use coin_mixer_challenge::{address, store::Store};

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(long)]
    api_url: url::Url,
}

fn main() -> io::Result<()> {
    let Options { api_url } = Options::from_args();
    println!("Given API URL:");
    println!("{}", api_url);
    let mut w_addrs = Vec::new();
    for addr_str in io::stdin().lock().lines() {
        w_addrs.push(address::Withdrawal::new(
            addr_str.expect("able to read from stdin"),
        ));
    }
    println!("Read the following withdrawal addresses:");
    for w_addr in &w_addrs {
        println!("{}", w_addr.to_string());
    }
    let mut store = Store::new();
    let d_addr = store.register(&w_addrs);
    println!("Generated deposit address:");
    println!("{}", d_addr.to_string());
    Ok(())
}
