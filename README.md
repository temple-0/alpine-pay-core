# Alpine Pay
## Introduction
Alpine is a web application payment platform built on the Osmosis blockchain; designed to facilitate tipping of content creators using Osmosis tokens, users can now show their appreciation by making secure donations while including personalized messages to any social media influencer or content creator they wish to support.

Alpine makes tipping content creators fun, simple, visually pleasing, and censorship-resistant. Alpine provides a seamless user experience by providing a Kado widget, so that users who are unfamiliar with the Cosmos ecosystem can easily purchase crypto with fiat currency and use Alpine to send it to their favorite influencer without the hassle of relying on centralized exchanges. Alpine also provides integrations for popular social media platforms such as Twitter, Instagram, TikTok, and others so that users can easily identify their favorite influencer in the app.

# Alpine Pay Core Contract
## Introduction
The Alpine Pay Core Contract provides all of the core functionality for Alpine. It facilitates the creation and processing of tips, ensuring the secure transfer of Osmosis from the sender to the content creator's wallet address. Additionally, it enables the inclusion of personalized messages, allowing users to express their sentiments to the content creators.

The Alpine Pay Core Contract does not facilitate the storage of social media data due to the lack of confidentiality of data stored on the blockchain. It also does not facilitate the purchase of crypto with fiat currency. These features are provided by the Alpine Pay Frontend.

## Usage
All usage of the Alpine Pay Core Contract assumes that you have a proper development environment set up for a Cosmos chain. **The following documentation will assume that you are using Osmosis in the Testnet environment**, but technically you can use other Cosmos chains which are compatible with the CosmWasm code used in the Alpine Pay Core Contract, such as Juno. For more information on setting up your development environment, see the documentation for your chosen chain.

### Instantiation
The first step of using the Alpine Pay Core Contract is to deploy it and instantiate it. The instantiation message for this contract takes no arguments.
1. Set the client configuration for osmosisd by navigating to `~/.osmosisd/config/client.toml`, then setting `node = "https://rpc.osmotest5.osmosis.zone:443"` and `chain-id = "osmo-test-5"`.

2. Navigate to the `contracts/alpine-pay` directory and build/optimize your code using
```
sudo docker run --rm -v "$(pwd)":/code \
     --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
     --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
     cosmwasm/rust-optimizer:0.12.6
```
3. Next, store your compiled code on the testnet blockchain and save the id of your code in an environment variable to use later.
```
id=$(osmosisd tx wasm store artifacts/alpine_pay.wasm  --from <your-osmosis-wallet-name> --gas-prices 0.1uosmo --gas auto --gas-adjustment 1.3 -y --output json -b block | jq -r '.logs[0].events[-1].attributes[1].value')
```
4. Instantiate the contract so that it can actually be used.
```
osmosisd tx wasm instantiate $id '{}' --from <your-osmosis-wallet-name> --label "migrate to osmo" --gas-prices 0.025uosmo --gas auto --gas-adjustment 1.3 -y -b block --admin <your-osmosis-wallet-address>
```
5. Grab the address of the contract.
```
address=$(osmosisd query wasm list-contract-by-code $id --output json | jq -r '.contracts[0]')
```
### Migration
If you want to update the code of an Alpine Core Contract deployment, then you'll need to migrate it. Migration can only be done from the `admin` address defined in the instantiation section. Additionally, this section assumes that you still have the address of the contract saved in the `$address` environment variable on your terminal.
1. Navigate to the `contracts/alpine-pay` directory and build/optimize your code using
```
sudo docker run --rm -v "$(pwd)":/code \
     --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
     --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
     cosmwasm/rust-optimizer:0.12.6
```
2. Next, store your compiled code on the testnet blockchain and save the id of your code in an environment variable to use later.
```
id=$(osmosisd tx wasm store artifacts/alpine_pay.wasm  --from <your-osmosis-wallet-name> --gas-prices 0.1uosmo --gas auto --gas-adjustment 1.3 -y --output json -b block | jq -r '.logs[0].events[-1].attributes[1].value')
```
3. Migrate the contract address to the new code ID.
```
osmosisd tx wasm migrate $address $id '{ }' --from <your-osmosis-wallet-name> --gas-prices 0.1uosmo --gas-adjustment 1.3 --gas auto -b block -y
```
4. To verify that the transaction was successful, you can run the following command.
```
osmosisd query wasm contract-history $address
```
### Register a User
Alpine allows users to register a username associated with their wallet address. This feature makes it easy to communicate with other users, because there's no need to memorize or copy a complicated wallet address. 
1. Verify that your desired username is available.
```
osmosisd query wasm contract-state smart $address '{"is_username_available":{"username":"<your-desired-username>"}}'
```
A username which is already taken should return `is_available: false`

2. Register your user.
```
osmosisd tx wasm execute $address '{"save_username":{"user":{"address":"<your-osmosis-wallet-address>", "username":""}, "username":"<your-desired-username>"}}' \
    --from <your-osmosis-wallet-name> \
    --gas auto --gas-adjustment 1.3 --gas-prices 0.1uosmo -b block
```
3. Verify that registration was successful.
```
osmosisd query wasm contract-state smart $address '{"get_user_by_name": {"username":"<your-chosen-username>"}}'
```
The output of this should return your address and chosen username.
### Send a Donation
The primary functionality of the Core Contract from the perspective of most users is sending donations. This functionality assumes that there are at least two users registered, as you can't send a donation to yourself.
1. Get a list of all users so that you can find who you want to send a donation to.
```
osmosisd query wasm contract-state smart $address '{"get_all_users": { }}'
```
2. Find the username of the user that you want to send the user to. Then send them a donation.
```
osmosisd tx wasm execute $address '{"send_donation":{"sender":"<your-username>", "recipient":"<recipient-username>", "message":"<your-message-text>"}}' --from <your-osmosis-wallet-name> --amount <your-desired-donation-amount> --gas auto --gas-adjustment 1.3 --gas-prices 0.1uosmo -b block
```
3. Verify that your donation was sent successfully
```
osmosisd query wasm contract-state smart $address '{"get_sent_donations":{"sender":"<your-username>"}}'
```
### Get a List of Donations Sent to You
From the perspective of a content creator, the biggest function in the Core Contract is viewing the donations that they've received. This assumes that you're already registered.
1. Query all of the donations sent to you
```
osmosisd query wasm contract-state smart $address '{"get_received_donations":{"recipient":"<your-username>"}}'
```
### Supporting Functionality
In addition to the main functions of the contract, there are a few other functions which support our web application. These typically wouldn't be used if you're using the CLI, but they could be interesting regardless.
- Obtain a count of all donations sent through the Core Contract.
```
osmosisd query wasm contract-state smart $address '{"get_num_donations":{ }}'
```
- Get a user by their wallet address
```
osmosisd query wasm contract-state smart $address '{"get_user_by_address": {"username":"<user-osmosis-wallet-address>"}}'
```