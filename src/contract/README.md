## Orderbook contract deployment
    near dev-deploy contract.wasm
## Orderbook contract init
    near call $ORDERBOOK new '{"owner_id": "'$ORDERBOOK'"}' --accountId $ORDERBOOK

## Orderbook create market
    near call $ORDERBOOK create_market '{"req":{"market": "BTC-USDT", "base_ft":"btc1.fsy412.testnet", "quote_ft":"usdt.fsy412.testnet"}
}' --accountId $ORDERBOOK

## Orderbook register in FT contract
    near call $FT_CONTRACT storage_deposit '{"account_id": "'$ORDERBOOK'"}' --accountId $FT_CONTRACT --amount 0.1

    near call usdt.fsy412.testnet storage_deposit '{"account_id": "'$ORDERBOOK'"}' --accountId usdt.fsy412.testnet --amount 0.1
    near call btc1.fsy412.testnet storage_deposit '{"account_id": "'$ORDERBOOK'"}' --accountId btc1.fsy412.testnet --amount 0.1

## User account register in FT contract
    near call $FT_CONTRACT storage_deposit '{"account_id": "fsy412.testnet"}' --accountId $FT_CONTRACT --amount 0.1

## User account register in Orderbook contract
    near call $ORDERBOOK storage_deposit '{"account_id": "fsy412.testnet"}' --accountId fsy412.testnet --amount 0.1
## Transfer some FT token to user account
    near call $FT_CONTRACT ft_transfer '{"receiver_id": "fsy412.testnet", "amount": "50000000000000000000000000", "memo": "Go Team!"}' --accountId $FT_CONTRACT --depositYocto 1
## User deposit FT token to orderbook contract
    near call $FT_CONTRACT ft_transfer_call '{"receiver_id": "'$ORDERBOOK'", "amount": "10000000000000000000000000", "msg": "Wooooooo!"}' --accountId fsy412.testnet --depositYocto 1 --gas 200000000000000

## Check user deposited balance
    near view $ORDERBOOK ft_deposits_of '{"account_id": "fsy412.testnet"}'
    
    near view $ORDERBOOK ft_deposits_of_asset '{"ft":"BTC","account_id": "fsy412.testnet"}'
 
## User withdraw FT from orderbook contract
    near call $ORDERBOOK ft_withdraw '{"amount": "10000000000000000000000000", "ft_contract_id":"dev-1668653145937-68979006040599"}' --accountId fsy412.testnet --depositYocto 1 --gas 300000000000000

# Contracts
- FT contract
  
   dev-1668653145937-68979006040599
- orderbook
  
   dev-1666537631161-54643597041874