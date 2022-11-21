mod account;
mod asset;
mod errors;
mod external;
mod ft_balances;
mod market;
mod owner;
mod settlement;
mod storage;
mod types;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::{
    assert_one_yocto, env, near_bindgen, AccountId, Balance, BorshStorageKey, PanicOnDefault,
};

extern crate events;
use crate::events::*;
use account::*;
use market::*;
use orderbook::{orders, Failed, L2OpenLimitOrderView, Order, OrderSide, Success};
use std::collections::HashMap;
use types::*;


fn get_current_time() -> u64 {
    return env::block_timestamp();
}

fn parse_side(side: &str) -> Option<OrderSide> {
    match side {
        "Ask" => Some(OrderSide::Ask),
        "Bid" => Some(OrderSide::Bid),
        _ => None,
    }
}
// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,

    pub markets: LookupMap<String, Market>,

    pub accounts: LookupMap<AccountId, Account>,

    // <dev-xxxxxxx,"BTC">
    pub ft_address: HashMap<String, String>,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    StorageDeposits,
    FTDeposits,
    Market,
    Account,
    FT,
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    /*
        initialization function (can only be called once).
        this initializes the contract with default data and the owner ID
        that's passed in
    */
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        let this = Self {
            owner_id,
            // ft_deposits: LookupMap::new(StorageKey::FTDeposits),
            markets: LookupMap::new(StorageKey::Market),
            accounts: LookupMap::new(StorageKey::Account),
            ft_address: HashMap::new(),
        };

        this
    }

    pub fn create_market(&mut self, req: CreateMarket) {
        let arr: Vec<&str> = req.market.split("-").collect();
        let m = Market::new(arr[0].to_string(), arr[1].to_string());
        self.markets.insert(&req.market.to_string(), &m);

        self.ft_address.insert(req.base_ft, arr[0].to_string());
        self.ft_address.insert(req.quote_ft, arr[1].to_string());
    }

    pub fn orderbook(&self) -> (Vec<L2OpenLimitOrderView>, Vec<L2OpenLimitOrderView>) {
        let market = self.markets.get(&"BTC-USDT".to_string()).unwrap();
        let orderbook = market.get_orderbook();
        orderbook
    }

    pub fn place_order(&mut self, req: PlaceOrder) {
        // create order requests
        // let order_asset = parse_asset("BTC").unwrap();
        // let price_asset = parse_asset("USD").unwrap();

        let market = &mut self.markets.get(&req.market).unwrap();
        match req.order_type.as_str() {
            "limit" => {
                let account_id = env::predecessor_account_id();
                let order = orders::new_limit_order_request(
                    BrokerAsset::BTC,
                    BrokerAsset::USDT,
                    parse_side(&req.side).unwrap(),
                    req.price,
                    req.qty,
                    get_current_time(),
                    account_id.to_string(),
                );

                let res = market.place_order(order);
                println!("res {:?}", res);
                self.settle_result(account_id, market, parse_side(&req.side).unwrap(), res);
                self.markets.insert(&"BTC-USDT".to_string(), &market);
            }
            "market" => {
                let account = env::predecessor_account_id();
                let order = orders::new_market_order_request(
                    BrokerAsset::BTC,
                    BrokerAsset::USDT,
                    parse_side(&req.side).unwrap(),
                    req.qty,
                    get_current_time(),
                    account.clone(),
                );

                let res = market.place_order(order);
                println!("res {:?}", res);
                self.settle_result(account, market, parse_side(&req.side).unwrap(), res);
                self.markets.insert(&"BTC-USDT".to_string(), &market);
            }
            _ => {}
        }

        emit_event(EventType::MyEvent(NewMyEvent {
            order_id: "test".to_string(),
        }))
    }

    pub fn get_orderbook(
        &self,
        market: String,
    ) -> (Vec<L2OpenLimitOrderView>, Vec<L2OpenLimitOrderView>) {
        let market_inst = &mut self.markets.get(&market).unwrap();
        market_inst.get_orderbook()
    }

    pub fn get_open_orders(&self, market: String) -> Vec<Order<BrokerAsset>> {
        let account_id = env::predecessor_account_id();
        let account = self.internal_get_account(account_id).unwrap();
        let orders = account.get_open_orders(market);
        orders
    }

    pub fn ft_deposits_of_asset(&self, ft: String, account_id: AccountId) -> U128 {
        let account = self.internal_get_account(account_id);
        match account {
            Some(account) => {
                let asset = parse_asset(&ft).unwrap();
                account.get_asset_balance(asset).into()
            }
            None => 0.into(),
        }
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use crate::storage::StorageManagement;

    use super::types::*;
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    pub fn set_predecessor_context(account_id: AccountId) {
        let context = VMContextBuilder::new()
            .predecessor_account_id(account_id)
            .build();
        testing_env!(context);
    }

    #[test]
    fn test_new() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new(accounts(1));
    }
    #[test]
    fn test_place_order() {
        println!("test place order");
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1));

        contract.place_order(PlaceOrder {
            market: "BTC-USDT".to_string(),
            side: "Ask".to_string(),
            price: 12,
            qty: 1,
            order_type: "market".to_string(),
        });
        let orderbook = contract.get_orderbook("BTC-USDT".to_string());
    }
    #[test]
    fn test_limit_order() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1));

        contract.create_market(CreateMarket {
            market: "BTC-USDT".to_string(),
            base_ft: "btc".to_string(),
            quote_ft: "usdt".to_string(),
        });

        let maker_account_id = accounts(1);
        let taker_account_id = accounts(2);

        let mut maker_acc = contract.internal_register_account(&maker_account_id);
        maker_acc.deposit_asset(BrokerAsset::BTC, 1);
        maker_acc.deposit_asset(BrokerAsset::USDT, 0);
        contract.accounts.insert(&maker_account_id, &maker_acc);
        let mut taker_acc = contract.internal_register_account(&taker_account_id);
        taker_acc.deposit_asset(BrokerAsset::BTC, 0);
        taker_acc.deposit_asset(BrokerAsset::USDT, 20000);
        contract.accounts.insert(&taker_account_id, &taker_acc);
        println!(
            "maker: {:?}, {:?}",
            maker_acc.get_asset_balance(BrokerAsset::BTC),
            maker_acc.get_asset_balance(BrokerAsset::USDT),
        );
        println!(
            "taker: {:?}, {:?}",
            taker_acc.get_asset_balance(BrokerAsset::BTC),
            taker_acc.get_asset_balance(BrokerAsset::USDT),
        );

        contract.place_order(PlaceOrder {
            market: "BTC-USDT".to_string(),
            side: "Ask".to_string(),
            price: 20000,
            qty: 1,
            order_type: "limit".to_string(),
        });
        // change predecessor_account_id
        set_predecessor_context(taker_account_id.clone());
        contract.place_order(PlaceOrder {
            market: "BTC-USDT".to_string(),
            side: "Bid".to_string(),
            price: 20000,
            qty: 1,
            order_type: "limit".to_string(),
        });

        println!("after settle");

        let maker_acc = contract.internal_get_account(maker_account_id).unwrap();
        let taker_acc = contract.internal_get_account(taker_account_id).unwrap();
        println!(
            "maker: {:?}, {:?}",
            &maker_acc.get_asset_balance(BrokerAsset::BTC),
            &maker_acc.get_asset_balance(BrokerAsset::USDT)
        );
        println!(
            "taker: {:?}, {:?}",
            &taker_acc.get_asset_balance(BrokerAsset::BTC),
            &taker_acc.get_asset_balance(BrokerAsset::USDT)
        );

        let orderbook = contract.get_orderbook("BTC-USDT".to_string());
        println!("{:?}", orderbook);
    }

    #[test]
    fn test_market_order() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1));

        let maker_account_id = accounts(1);
        let taker_account_id = accounts(2);

        let mut maker_acc = contract.internal_register_account(&maker_account_id);
        maker_acc.deposit_asset(BrokerAsset::BTC, 1);
        maker_acc.deposit_asset(BrokerAsset::USDT, 0);
        contract.accounts.insert(&maker_account_id, &maker_acc);
        let mut taker_acc = contract.internal_register_account(&taker_account_id);
        taker_acc.deposit_asset(BrokerAsset::BTC, 0);
        taker_acc.deposit_asset(BrokerAsset::USDT, 20000);
        contract.accounts.insert(&taker_account_id, &taker_acc);
        println!(
            "maker: {:?}, {:?}",
            maker_acc.get_asset_balance(BrokerAsset::BTC),
            maker_acc.get_asset_balance(BrokerAsset::USDT),
        );
        println!(
            "taker: {:?}, {:?}",
            taker_acc.get_asset_balance(BrokerAsset::BTC),
            taker_acc.get_asset_balance(BrokerAsset::USDT),
        );

        contract.place_order(PlaceOrder {
            market: "BTC-USDT".to_string(),
            side: "Ask".to_string(),
            price: 20000,
            qty: 1,
            order_type: "limit".to_string(),
        });
        // change predecessor_account_id
        set_predecessor_context(taker_account_id.clone());
        contract.place_order(PlaceOrder {
            market: "BTC-USDT".to_string(),
            side: "Bid".to_string(),
            price: 20000,
            qty: 1,
            order_type: "market".to_string(),
        });

        println!("after settle");

        let maker_acc = contract.internal_get_account(maker_account_id).unwrap();
        let taker_acc = contract.internal_get_account(taker_account_id).unwrap();
        println!(
            "maker: {:?}, {:?}",
            &maker_acc.get_asset_balance(BrokerAsset::BTC),
            &maker_acc.get_asset_balance(BrokerAsset::USDT)
        );
        println!(
            "taker: {:?}, {:?}",
            &taker_acc.get_asset_balance(BrokerAsset::BTC),
            &taker_acc.get_asset_balance(BrokerAsset::USDT)
        );

        let orderbook = contract.get_orderbook("BTC-USDT".to_string());
        println!("{:?}", orderbook);
    }

    #[test]
    fn test_market_order_unmatched() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1));

        let maker_account_id = accounts(1);
        let taker_account_id = accounts(2);

        let mut maker_acc = contract.internal_register_account(&maker_account_id);
        maker_acc.deposit_asset(BrokerAsset::BTC, 1);
        maker_acc.deposit_asset(BrokerAsset::USDT, 0);
        contract.accounts.insert(&maker_account_id, &maker_acc);
        let mut taker_acc = contract.internal_register_account(&taker_account_id);
        taker_acc.deposit_asset(BrokerAsset::BTC, 0);
        taker_acc.deposit_asset(BrokerAsset::USDT, 20000);
        contract.accounts.insert(&taker_account_id, &taker_acc);
        println!(
            "maker: {:?}, {:?}",
            maker_acc.get_asset_balance(BrokerAsset::BTC),
            maker_acc.get_asset_balance(BrokerAsset::USDT),
        );
        println!(
            "taker: {:?}, {:?}",
            taker_acc.get_asset_balance(BrokerAsset::BTC),
            taker_acc.get_asset_balance(BrokerAsset::USDT),
        );

        contract.place_order(PlaceOrder {
            market: "BTC-USDT".to_string(),
            side: "Ask".to_string(),
            price: 20000,
            qty: 1,
            order_type: "market".to_string(),
        });
        let orderbook = contract.get_orderbook("BTC-USDT".to_string());
        println!("{:?}", orderbook);
    }

    #[test]
    fn test_match() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1));

        contract.place_order(PlaceOrder {
            market: "BTC-USDT".to_string(),
            side: "Ask".to_string(),
            price: 12,
            qty: 1,
            order_type: "market".to_string(),
        });

        contract.place_order(PlaceOrder {
            market: "BTC-USDT".to_string(),
            side: "Bid".to_string(),
            price: 12,
            qty: 1,
            order_type: "market".to_string(),
        });

        let orderbook = contract.get_orderbook("BTC-USDT".to_string());
        println!("{:?}", orderbook);
    }

    #[test]
    fn test_admin() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1));

        let mut req = Vec::new();
        req.push(RegisterTokenElement {
            ft: "BTC".to_string(),
            address: "BTC".to_string(),
        });

        contract.register_ft_address(req)
    }

    #[test]
    fn test_account_balance() {
        let mut context = get_context(accounts(1));

        testing_env!(context.build());

        context.attached_deposit(500000000000000000000000);
        let mut contract = Contract::new(accounts(1));
        let account_id = accounts(1);
        contract.storage_deposit(Some(account_id), Some(true));

        let account_id = accounts(1);

        // let account = contract.internal_get_account(account_id).unwrap();
        let mut account = contract.accounts.get(&account_id).unwrap();
        account.deposit_asset(BrokerAsset::BTC, 12);

        contract.accounts.insert(&account_id, &account);

        let account_id = accounts(1);
        let balance = contract.ft_deposits_of_asset("BTC".to_string(), account_id);
        println!("balance:{:?}", balance)
    }
}
