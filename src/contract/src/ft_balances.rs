use crate::*;
use near_contract_standards::fungible_token::core_impl::ext_fungible_token;
use near_sdk::json_types::U128;
use near_sdk::{ext_contract, require, Gas, PromiseResult};

const GAS_FOR_RESOLVE_REFUND: Gas = Gas(30_000_000_000_000);
const GAS_FOR_FT_TRANSFER: Gas = Gas(20_000_000_000_000);
pub const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(20_000_000_000_000); // 20 TGas
#[ext_contract(ext_self)]
pub trait Exchange {
    fn exchange_callback_post_withdraw(
        &mut self,
        ft_contract_id: AccountId,
        receiver_id: AccountId,
        amount: U128,
    );
}

trait FungibleTokenReceiver {
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128) -> U128;

    fn ft_withdraw(&mut self, amount: U128, ft_contract_id: AccountId);

    fn resolve_refund(&mut self, caller: AccountId, amount: U128) -> U128;

    fn ft_deposits_of(&self, account_id: AccountId) -> U128;
}

//implementation of the trait
#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    /// This is how users will fund their FT balances in the contract
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128) -> U128 {
        // get the contract ID which is the predecessor
        let ft_contract_id = env::predecessor_account_id();
        //get the signer which is the person who initiated the transaction
        let signer_id = env::signer_account_id();

        // Add the amount to the user's current balance
        let account = &mut self.accounts.get(&signer_id).unwrap();
        let asset = self.ft_contract_id_to_asset(ft_contract_id);
        account.deposit_asset(asset, amount.0);

        self.accounts.insert(&sender_id, account);

        U128(amount.0)
    }

    #[payable]
    fn ft_withdraw(&mut self, amount: U128, ft_contract_id: AccountId) {
        //make sure the user attaches exactly 1 yoctoNEAR for security purposes.
        //this will redirect them to the NEAR wallet (or requires a full access key).
        assert_one_yocto();
        // Get the receiver_id and ensure they have enough balance
        let caller = env::predecessor_account_id();
        // let cur_bal = self.ft_deposits.get(&caller).unwrap_or(0);
        // require!(cur_bal >= amount.0, "Insufficient balance");
        // Subtract the amount from the caller's balance
        // let new_bal = cur_bal - amount.0;
        // self.ft_deposits.insert(&caller, &new_bal);
        ext_fungible_token::ft_transfer(
            caller.clone(),
            amount,
            None,
            ft_contract_id.clone(),
            1,
            GAS_FOR_FT_TRANSFER,
        )
        .then(ext_self::exchange_callback_post_withdraw(
            ft_contract_id.clone(),
            caller.clone(),
            U128(amount.0),
            env::current_account_id(),
            0,
            GAS_FOR_RESOLVE_TRANSFER,
        ));

        let account = &mut self.accounts.get(&caller).unwrap();
        let asset = self.ft_contract_id_to_asset(ft_contract_id);
        account.withdraw_asset(asset, amount.0);
    }

    #[private]
    fn resolve_refund(&mut self, caller: AccountId, amount: U128) -> U128 {
        let amount: Balance = amount.into();

        // Get the amount to revert the caller's balance with
        let revert_amount = match env::promise_result(0) {
            PromiseResult::NotReady => env::abort(),
            // If the promise was successful, get the return value and cast it to a U128.
            PromiseResult::Successful(_) => 0,
            // If the promise wasn't successful, return the original amount.
            PromiseResult::Failed => amount,
        };

        if revert_amount > 0 {
            // Get the caller's current balance
            // let cur_bal = self.ft_deposits.get(&caller).unwrap_or(0);
            // Add the amount to the caller's balance
            // let new_bal = cur_bal + revert_amount;
            // self.ft_deposits.insert(&caller, &new_bal);
        }

        U128(revert_amount)
    }

    /// Get the amount of FTs the user has deposited into the contract
    fn ft_deposits_of(&self, account_id: AccountId) -> U128 {
        // self.ft_deposits.get(&account_id).unwrap_or(0).into()
        0.into()
    }
}

#[near_bindgen]
impl Contract {
    #[private]
    pub fn exchange_callback_post_withdraw(
        &mut self,
        token: AccountId,
        receiver_id: AccountId,
        amount: U128,
    ) {
    }
}
