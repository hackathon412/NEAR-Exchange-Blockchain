use crate::OrderSide;
use crate::*;
use near_sdk::AccountId;
use std::str::FromStr;

impl Contract {
    pub fn settle_result(
        &mut self,
        taker_account_id: AccountId,
        market: &mut Market,
        taker_side: OrderSide,
        results: Vec<Result<Success, Failed>>,
    ) {
        for temp_variable in &results {
            let _ret = match temp_variable {
                Ok(success) => {
                    // let success = temp_variable.as_ref().unwrap();
                    match success {
                        Success::Accepted {
                            id: _,
                            order_type: _,
                            order_creator: _,
                            ts: _,
                        } => {

                            
                        }
                        Success::Filled {
                            order_id: u64,
                            side,
                            order_type: _,
                            price,
                            qty,
                            order_creator,
                            ts: _,
                        } => {
                            match side {
                                OrderSide::Ask => {
                                    match taker_side {
                                        OrderSide::Ask => continue,
                                        OrderSide::Bid => {
                                            // taker buy
                                            let maker_account_id =
                                                AccountId::from_str(order_creator).unwrap();
                                            if maker_account_id == taker_account_id {
                                                continue;
                                            }
                                            let taker_account =
                                                &mut self.accounts.get(&taker_account_id).unwrap();
                                            let maker_account =
                                                &mut self.accounts.get(&maker_account_id).unwrap();

                                            taker_account.withdraw_asset(
                                                market.quote_token,
                                                *qty * (*price),
                                            );
                                            taker_account.deposit_asset(market.base_token, *qty);

                                            maker_account
                                                .deposit_asset(market.quote_token, *qty * (*price));
                                            maker_account.withdraw_asset(market.base_token, *qty);

                                            self.accounts.insert(&taker_account_id, taker_account);
                                            self.accounts.insert(&maker_account_id, maker_account);
                                        }
                                    }
                                }
                                OrderSide::Bid => {
                                    match taker_side {
                                        OrderSide::Bid => continue,
                                        OrderSide::Ask => {
                                            // taker sell
                                            let maker_account_id =
                                                AccountId::from_str(order_creator).unwrap();
                                            if maker_account_id == taker_account_id {
                                                continue;
                                            }

                                            let taker_account =
                                                &mut self.accounts.get(&taker_account_id).unwrap();
                                            let maker_account =
                                                &mut self.accounts.get(&maker_account_id).unwrap();

                                            taker_account.withdraw_asset(market.base_token, *qty);
                                            taker_account
                                                .deposit_asset(market.quote_token, *qty * (*price));

                                            maker_account.deposit_asset(market.base_token, *qty);
                                            maker_account.withdraw_asset(
                                                market.quote_token,
                                                *qty * (*price),
                                            );

                                            self.accounts.insert(&taker_account_id, taker_account);
                                            self.accounts.insert(&maker_account_id, maker_account);
                                        }
                                    }
                                }
                            };
                        }
                        Success::PartiallyFilled {
                            order_id: _,
                            side,
                            order_type: _,
                            price: _,
                            qty,
                            order_creator,
                            ts: _,
                        } => {}
                        Success::Amended {
                            id: _,
                            price: _,
                            qty: _,
                            ts: _,
                        } => {}
                        Success::Cancelled { id: _, ts: _ } => {}
                    }
                }
                Err(error) => {
                    println!("{:?}", error)
                }
            };
        }
    }
}
