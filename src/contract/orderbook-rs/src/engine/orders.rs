use super::domain::OrderSide;
use near_sdk::AccountId;
use std::fmt::Debug;

#[derive(Debug)]
pub enum OrderRequest<Asset>
where
    Asset: Debug + Clone,
{
    NewMarketOrder {
        order_asset: Asset,
        price_asset: Asset,
        side: OrderSide,
        qty: u128,
        ts: u64,
        account_id: AccountId,
    },

    NewLimitOrder {
        order_asset: Asset,
        price_asset: Asset,
        side: OrderSide,
        price: u128,
        qty: u128,
        ts: u64,
        account: String,
    },

    AmendOrder {
        id: u64,
        side: OrderSide,
        price: u128,
        qty: u128,
        ts: u64,
    },

    CancelOrder {
        id: u64,
        side: OrderSide,
        //ts: SystemTime,
    },
}

/* Constructors */

/// Create request for the new market order
pub fn new_market_order_request<Asset>(
    order_asset: Asset,
    price_asset: Asset,
    side: OrderSide,
    qty: u128,
    ts: u64,
    account_id: AccountId,
) -> OrderRequest<Asset>
where
    Asset: Debug + Clone,
{
    OrderRequest::NewMarketOrder {
        order_asset,
        price_asset,
        qty,
        side,
        ts,
        account_id,
    }
}

/// Create request for the new limit order
pub fn new_limit_order_request<Asset>(
    order_asset: Asset,
    price_asset: Asset,
    side: OrderSide,
    price: u128,
    qty: u128,
    ts: u64,
    account: String,
) -> OrderRequest<Asset>
where
    Asset: Debug + Clone,
{
    OrderRequest::NewLimitOrder {
        order_asset,
        price_asset,
        side,
        price,
        qty,
        ts,
        account,
    }
}

/// Create request for changing price/qty for the active limit order.
///
/// Note: do not change order side!
/// Instead cancel existing order and create a new one.
pub fn amend_order_request<Asset>(
    id: u64,
    side: OrderSide,
    price: u128,
    qty: u128,
    ts: u64,
) -> OrderRequest<Asset>
where
    Asset: Debug + Clone,
{
    OrderRequest::AmendOrder {
        id,
        side,
        price,
        qty,
        ts,
    }
}

/// Create request for cancelling active limit order
pub fn limit_order_cancel_request<Asset>(order_id: u64, side: OrderSide) -> OrderRequest<Asset>
where
    Asset: Debug + Clone,
{
    OrderRequest::CancelOrder { id: order_id, side }
}
