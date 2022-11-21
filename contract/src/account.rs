use crate::errors;
use crate::types::{parse_asset, BrokerAsset};
use crate::Contract;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, Balance, StorageUsage};
use orderbook::Order;
use std::collections::HashMap;

const U128_STORAGE: StorageUsage = 16;
const U64_STORAGE: StorageUsage = 8;
const U32_STORAGE: StorageUsage = 4;
/// max length of account id is 64 bytes. We charge per byte.
const ACC_ID_STORAGE: StorageUsage = 64;
/// As a key, 4 bytes length would be added to the head
const ACC_ID_AS_KEY_STORAGE: StorageUsage = ACC_ID_STORAGE + 4;
const KEY_PREFIX_ACC: StorageUsage = 64;
/// As a near_sdk::collection key, 1 byte for prefiex
const ACC_ID_AS_CLT_KEY_STORAGE: StorageUsage = ACC_ID_AS_KEY_STORAGE + 1;

// ACC_ID: the Contract accounts map key length
// + U128_STORAGE: near_amount storage
// + U32_STORAGE: tokens HashMap length
// + U32_STORAGE: open_orders HashMap length
// + U64_STORAGE: storage_used
pub const INIT_ACCOUNT_STORAGE: StorageUsage =
    ACC_ID_AS_CLT_KEY_STORAGE + U128_STORAGE + U32_STORAGE + U32_STORAGE + U64_STORAGE;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Account {
    balance: HashMap<String, Balance>,
    /// Amounts of tokens and native NEAR deposited to this account.
    asset_balance: HashMap<BrokerAsset, Balance>,
    /// account's open orders.
    open_orders: HashMap<String, HashMap<u64, Order<BrokerAsset>>>,
    /// Amount of NEAR deposited for storage. This is distinct from NEAR
    /// available for trading.
    pub storage_balance: Balance,
}

impl Account {
    pub fn new() -> Self {
        Self {
            balance: HashMap::new(),
            asset_balance: HashMap::new(),
            open_orders: HashMap::new(),
            storage_balance: 0,
        }
    }
    pub fn deposit(&mut self, asset: String, balance: Balance) {
        let exists = self.balance.contains_key(&asset);
        if exists {
            *self.balance.get_mut(&asset).unwrap() += balance;
        } else {
            self.balance.insert(asset, balance);
        }
    }

    pub fn withdraw(&mut self, asset: String, balance: Balance) {
        let exists = self.balance.contains_key(&asset);
        if exists {
            let val = self.balance.get(&asset).unwrap();
            *self.balance.get_mut(&asset).unwrap() = val - balance;
        } else {
        }
    }

    pub fn get_balance(&self, asset: String) -> Balance {
        self.balance[&asset]
    }

    pub fn deposit_asset(&mut self, asset: BrokerAsset, balance: Balance) {
        if let Some(x) = self.asset_balance.get(&asset).cloned() {
            self.asset_balance.insert(asset, balance + x);
        } else {
            self.asset_balance.insert(asset, balance);
        }
    }

    pub fn withdraw_asset(&mut self, asset: BrokerAsset, balance: Balance) {
        if let Some(x) = self.asset_balance.get(&asset).cloned() {
            if x < balance {
                env::panic_str(errors::INSUFFICIENT_BALANCE);
            }
            self.asset_balance.insert(asset, x - balance);
        } else {
            env::panic_str(errors::INSUFFICIENT_BALANCE);
        }
    }

    pub fn get_asset_balance(&self, asset: BrokerAsset) -> Balance {
        let balance = self.asset_balance.get(&asset).unwrap_or(&0);
        *balance
    }

    pub fn save_open_order(&mut self, market: String, order: Order<BrokerAsset>) {
        match self.open_orders.get_mut(&market) {
            Some(orders_in_market) => {
                orders_in_market.insert(order.order_id, order);
            }
            None => {
                let mut orders_in_market = HashMap::new();
                orders_in_market.insert(order.order_id, order);
                self.open_orders.insert(market.clone(), orders_in_market);
            }
        };
    }

    pub fn get_open_orders(&self, market: String) -> Vec<Order<BrokerAsset>> {
        self.open_orders[&market]
            .clone()
            .into_iter()
            .map(|x| x.1.clone())
            .collect()
    }

    /// Returns minimal account deposit storage usage possible.
    pub fn min_storage_usage() -> Balance {
        INIT_ACCOUNT_STORAGE as Balance * env::storage_byte_cost()
    }

    /// Returns amount of $NEAR necessary to cover storage used by this data structure.
    pub fn storage_usage(&self) -> Balance {
        (INIT_ACCOUNT_STORAGE
            + self.asset_balance.len() as u64
                * (KEY_PREFIX_ACC + ACC_ID_AS_KEY_STORAGE + U128_STORAGE)) as u128
            * env::storage_byte_cost()
    }

    /// Returns how much NEAR is available for storage.
    pub fn storage_available(&self) -> Balance {
        let locked = self.storage_usage();
        if self.storage_balance > locked {
            self.storage_balance - locked
        } else {
            0
        }
    }
}

impl Contract {
    pub fn internal_register_account(&mut self, account_id: &AccountId) -> Account {
        println!("internal_register_account:{:?}", account_id);
        let acc = Account::new();
        self.accounts.insert(account_id, &acc);
        acc
    }

    pub fn internal_get_account(&self, account_id: AccountId) -> Option<Account> {
        self.accounts.get(&account_id)
    }
}
mod tests {
    use super::*;
    use orderbook::OrderSide;
    use std::collections::HashMap;
    #[test]
    fn test_account() {
        let mut acc = Account::new();
        let bal: Balance = 12;
        acc.deposit("BTC".to_string(), bal);
        // println!("balance:{}", acc.get_balance("BTC".to_string()));
        assert_eq!(acc.get_balance("BTC".to_string()), bal);
    }

    #[test]
    fn test_account_order() {
        let mut account = Account::new();

        let order = Order {
            order_id: 1,
            order_asset: BrokerAsset::NEAR,
            price_asset: BrokerAsset::USDC,
            side: OrderSide::Bid,
            price: 12,
            qty: 1,
            account: "test".to_string(),
        };

        account.save_open_order("NEAR-USDT".to_string(), order)
    }

    #[test]
    fn test_account_balance() {
        let mut account = Account::new();
        let bal: Balance = 12;
        account.deposit_asset(BrokerAsset::NEAR, bal);
        // println!("balance:{}", &account.get_asset_balance(BrokerAsset::NEAR));
        assert_eq!(account.get_asset_balance(BrokerAsset::NEAR), bal);
    }

    #[test]
    fn account_withdraw() {
        let mut account = Account::new();
        let bal: Balance = 12;
        account.withdraw_asset(BrokerAsset::NEAR, bal);
        // println!("balance:{}", &account.get_asset_balance(BrokerAsset::NEAR));
        // assert_eq!(account.get_asset_balance(BrokerAsset::NEAR), bal);
    }

    #[test]
    fn get_open_order() {
        let mut account = Account::new();
        let order = Order {
            order_id: 1,
            order_asset: BrokerAsset::NEAR,
            price_asset: BrokerAsset::USDC,
            side: OrderSide::Bid,
            price: 12,
            qty: 1,
            account: "test".to_string(),
        };

        account.save_open_order("NEAR-USDT".to_string(), order);
        let orders = account.get_open_orders("NEAR-USDT".to_string());
        println!("{:?}", orders);
    }
}
