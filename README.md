# NEAR-Exchange-Blockchain

[NEAR MetaBUILD III Hackathon](https://metabuild.devpost.com/?ref_feature=challenge&ref_medium=discover) project for building an on-chain decentralized exchange on NEAR Protocol.

## Overview
 NEAR Exchange is a set of open source smart contracts for implementing NEP-14 token exchange capabilities on [NEAR Protocol](https://near.org).It is an on-chain orderbook based exchange that lets user trade fungible tokens (NEP-141) . User can place Market&Limit order on the exchange to trade the assets. The platform is entirely decentralized and is open source, you can be rest assured that your transactions are safe and trustless.


## Marching Engine
The smart contract implements a matching engine with NEAR's Rust smart contract language. It maintains two groups of orders, bid(buy) and ask(sell) orders with rust BinaryHeap data structure for fast searching inserting and prioritizing orders. The Matching Engine is generally responsible for determining if a set of two or more orders "match" and settle the orders.


## Project Setup 
### Compile and Deploy Contracts 
- cd contract
- ./build.sh
- ./deploy_testnet.sh


### Web
- cd src
- yarn && yarn serve
  
## Tech Stack Used
 - Vue
 - Rust
 - near-sdk-rs
 - near-api.js
 - Tailwind

## Screenshots
- Main page
![avatar](https://d112y698adiu2z.cloudfront.net/photos/production/software_photos/002/310/491/datas/original.jpg)

- Trading Page
![avatar](https://d112y698adiu2z.cloudfront.net/photos/production/software_photos/002/310/718/datas/original.jpg)



## [Demo Site](http://47.240.61.73)


## YouTube Demo

https://youtu.be/Mq8Bof4WNps


## Further Development Plan
- Implements an indexer for monitoring smart contract activities and record the activities accordingly.
- Persist the user trading history and marketing trading history to off-chain database for query.
- Optimize smart contract and improve UI & UX.  



