# Oracle-Infinity

### Feature
  - Work around with the timestamp & http fns in offchain-worker module, implement the features:
    1) fetch the real time when strat a new tx/request
    2) fetch the off-chain data via http-request
    
  - Interate with the real world
    1) the ability of connection, the local node can connect to PolkadotJS and send tx
       ```
       Developer setting:
       
       {
        "Kitty": "[u8;16]",
        "KittyIndex": "u32",
        "KittyLinkedItem": {
          "prev": "Option<KittyIndex>",
          "next": "Option<KittyIndex>"
        },
        "Value": "u32",
        "BlockNumber": "u32"
        }
       ```
    2) kitties app has been added into Polkadot UI just like others apps: accounts, staking. 
    
### More goals
    - To enbale more features
    - To slove the tackles
