extern crate zaif_api;

use zaif_api::AccessKey;
use zaif_api::public_api::*;
use zaif_api::trade_api::*;

fn main() {
    let api = CurrenciesBuilder::new().name("btc").finalize();
    println!("{}", api.exec().unwrap());

    let api = CurrencyPairsBuilder::new().finalize();
    println!("{}", api.exec().unwrap());

    let api = LastPriceBuilder::new().currency_pair("btc_jpy").finalize();
    println!("{}", api.exec().unwrap());

    let api = DepthBuilder::new().currency_pair("btc_jpy").finalize();
    println!("{}", api.exec().unwrap());

    let access_key = AccessKey::new("YOUR_API_KEY", "YOUR_API_SECRET");
    let api = GetInfo2::new(access_key.clone());
    println!("{}", api.exec().unwrap());

    let api = TradeBuilder::new(access_key.clone())
        .currency_pair("zaif_jpy")
        .action(TradeAction::Bid)
        .price(1.0)
        .amount(0.1)
        .finalize();
    println!("{}", api.exec().unwrap());
}
