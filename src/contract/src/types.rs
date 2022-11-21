use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PlaceOrder {
    pub market: String,
    pub side: String,
    pub price: u128,
    pub qty: u128,
    pub order_type: String,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct OrderbookReq {
    pub market: String,
    pub side: String,
    pub price: u128,
    pub qty: u128,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct DepositReq {
    pub market: String,
    pub qty: f64,
}

pub type RegisterToken = Vec<RegisterTokenElement>;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct RegisterTokenElement {
    #[serde(rename = "ft")]
    pub ft: String,

    #[serde(rename = "address")]
    pub address: String,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CreateMarket {
    #[serde(rename = "market")]
    pub market: String,

    #[serde(rename = "base_ft")]
    pub base_ft: String,

    #[serde(rename = "quote_ft")]
    pub quote_ft: String
}


#[derive(
    Eq,
    PartialEq,
    PartialOrd,
    BorshSerialize,
    BorshDeserialize,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Debug,
    Hash,
)]
#[serde(crate = "near_sdk::serde")]
pub enum BrokerAsset {
    USD,
    EUR,
    BTC,
    ETH,
    NEAR,
    USDC,
    USDT,
}

impl Default for BrokerAsset {
    fn default() -> Self {
        BrokerAsset::BTC
    }
}

pub fn parse_asset(asset: &str) -> Option<BrokerAsset> {
    match asset {
        "USD" => Some(BrokerAsset::USD),
        "EUR" => Some(BrokerAsset::EUR),
        "BTC" => Some(BrokerAsset::BTC),
        "ETH" => Some(BrokerAsset::ETH),
        "NEAR" => Some(BrokerAsset::NEAR),
        "USDC" => Some(BrokerAsset::USDC),
        "USDT" => Some(BrokerAsset::USDT),
        _ => None,
    }
}
