const ORDERBOOK_CONTRACT = 'dev-1669066106934-14317766527403'
const GAS = '300000000000000'

function getConfig(env) {
  switch (env) {
    case "mainnet":
      return {
        networkId: "mainnet",
        nodeUrl: "https://rpc.mainnet.near.org",
        walletUrl: "https://wallet.near.org",
        helperUrl: "https://helper.mainnet.near.org",
        explorerUrl: "https://explorer.mainnet.near.org",
        GAS: GAS,
      };
    case "testnet":
      return {
        networkId: "testnet",
        nodeUrl: "https://rpc.testnet.near.org",
        walletUrl: "https://wallet.testnet.near.org",
        helperUrl: "https://helper.testnet.near.org",
        explorerUrl: "https://explorer.testnet.near.org",
        orderbookContract: ORDERBOOK_CONTRACT,
        GAS: GAS,
        tokens: [
          {
            "chainId": 1,
            "address": "btc1.fsy412.testnet",
            "name": "Bit Coin",
            "symbol": "BTC",
            "decimals": 18,
            "logoURI": "https://assets.coingecko.com/coins/images/1/large/bitcoin.png?1547033579",
            "id": "bitcoin"
          },
          {
            "chainId": 1,
            "address": "usdt.fsy412.testnet",
            "name": "BSC Token",
            "symbol": "BNB",
            "decimals": 18,
            "logoURI": "https://trade.mango.markets/assets/icons/bnb.svg",
            "id": "binancecoin"
          },
          {
            "chainId": 1,
            "address": "usdt.fsy412.testnet",
            "name": "USDT",
            "symbol": "USDT",
            "decimals": 18,
            "logoURI": "https://assets.coingecko.com/coins/images/325/large/Tether-logo.png?1598003707"
          }
        ]
      };
    default:
      throw Error(
        `Unconfigured environment '${env}'. Can be configured in src/config.js.`
      );
  }
}

export default getConfig;
