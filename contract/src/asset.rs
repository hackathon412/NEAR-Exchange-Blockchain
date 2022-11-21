use crate::{types::{BrokerAsset, parse_asset}, Contract};
use near_sdk::{AccountId, env};

impl Contract {
    pub fn ft_contract_id_to_asset(&self, account_id: AccountId) -> BrokerAsset {
        let address = self.ft_address.get(&account_id.to_string());
        match address {
            Some(asset) => {
                parse_asset(&asset).unwrap()
            }
            None => {
                env::panic_str("unknown asset")
            }
        }
    }
}
