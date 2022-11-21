use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use orderbook::orders::OrderRequest;
use orderbook::*;
use crate::types::BrokerAsset;

#[derive(Default, BorshSerialize, BorshDeserialize)]
pub struct Market {
    pub base_token: BrokerAsset,
    pub quote_token: BrokerAsset,
    pub orderbook: Orderbook<BrokerAsset>,
}

impl Market {
    pub fn new(base: String, quote: String) -> Self {
        Self {
            base_token: BrokerAsset::BTC,
            quote_token: BrokerAsset::USDT,
            orderbook: Orderbook::new(BrokerAsset::BTC, BrokerAsset::USDT),
        }
    }

    pub fn place_order(&mut self, order: OrderRequest<BrokerAsset>) -> Vec<Result<Success, Failed>> {
        let res = self.orderbook.process_order(order);
        res
    }

    pub fn get_orderbook(&self) -> (Vec<L2OpenLimitOrderView>, Vec<L2OpenLimitOrderView>) {
        let bid = self
            .orderbook
            .bid_queue
            .clone()
            .idx_queue
            .unwrap()
            .iter()
            .map(|x| L2OpenLimitOrderView {
                price: x.price,
                qty: self.orderbook.bid_queue.get_order(x.id).qty,
            })
            .collect::<Vec<L2OpenLimitOrderView>>();

        let ask = self
            .orderbook
            .ask_queue
            .clone()
            .idx_queue
            .unwrap()
            .iter()
            .map(|x| L2OpenLimitOrderView {
                price: x.price,
                qty: self.orderbook.ask_queue.get_order(x.id).qty,
            })
            .collect::<Vec<L2OpenLimitOrderView>>();
        (bid, ask)
    }
}
#[cfg(test)]
mod tests {
    use near_sdk::env;

    use crate::parse_side;
    #[test]
    fn test_market() {
        use super::*;
        let mut market = Market::new("BTC".to_string(), "USDT".to_string());
        let order = orders::new_limit_order_request(
            BrokerAsset::BTC,
            BrokerAsset::USDT,
            parse_side(&"Bid").unwrap(),
            1,
            10,
            env::block_timestamp(),
            "test".to_string(),
        );
        market.place_order(order);
        market.get_orderbook();
    }
}
