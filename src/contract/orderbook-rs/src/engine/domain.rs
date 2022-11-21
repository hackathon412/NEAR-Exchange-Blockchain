use std::fmt::Debug;

extern crate near_sdk;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub enum OrderSide {
    Bid,
    Ask,
}

impl Default for OrderSide {
    fn default() -> Self {
        OrderSide::Bid
    }
}

// #[derive(Default, Debug, Clone, BorshDeserialize, BorshSerialize)]
// pub struct Asset;

#[derive(Default, Debug, Clone, BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Order<Asset>
where
    Asset: Debug + Clone,
{
    pub order_id: u64,
    pub order_asset: Asset,
    pub price_asset: Asset,
    pub side: OrderSide,
    pub price: u128,
    pub qty: u128,
    pub account: String,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum OrderType {
    Market,
    Limit,
}
