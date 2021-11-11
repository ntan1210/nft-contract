/*
 * This is an DEMO of a Rust smart contract with some simple, symmetric functions:
 *
 * 1. Mint token: mint an token to the blockchain with unique tokenId
 * 2. Set price: When users mint an token to the blockchain, they can set the price of that token and start selling it in the market
 * 3. Purchase: this function allows users to purchase some NFT from the market
 * 4. Transfer token: only the owner of the token can transfer the ownership of that token to other user
 * 5. Withdraw: After selling some NFT in the market, they can with draw the proceeding balace to their wallet
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, PanicOnDefault};
use near_sdk::{AccountId, Balance, Promise};
use std::cmp::PartialEq;

const MINT_FEE: Balance = 1_000_000_000_000_000_000_000_000; //Fee to mind one NFT to the blockchain
const OTHER_FEE: Balance = 1_000_000_000_000_000_000_000; //Fee to set the price, withdraw, etc

pub type TokenId = u64;
//Use string to simplify the demo
pub type TokenMetadata = String;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum TokenStatus {
    NotForSale,
    ForSale,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenData {
    pub owner_id: AccountId,
    pub status: TokenStatus,
    pub price: U128,
    pub metadata: TokenMetadata,
    pub token_id: TokenId,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    contract_owner: AccountId,                       //The owner of the contract
    nft_listing: LookupMap<AccountId, Vec<TokenId>>, //Mapping the NFT collections of each accountID
    search_token_data: LookupMap<TokenId, TokenData>, //Mapping data of each tokenID
    owner_balance: LookupMap<AccountId, U128>, //Mapping the balance that owners recieved from selling NFT
    last_token_id: TokenId,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Self {
            contract_owner: env::signer_account_id(),
            nft_listing: LookupMap::new(b"lis".to_vec()),
            search_token_data: LookupMap::new(b"srch".to_vec()),
            owner_balance: LookupMap::new(b"bl".to_vec()),
            last_token_id: 0,
        }
    }

    #[payable]
    pub fn mint_token(&mut self, owner_id: AccountId, metadata: TokenMetadata) -> TokenId {
        assert!(
            env::attached_deposit() == MINT_FEE,
            "Don't have enough balance"
        );
        self.mint(owner_id, metadata)
    }

    #[payable]
    pub fn set_price(&mut self, token_id: TokenId, price: U128) {
        let mut token = self.get_token_data(token_id);
        assert!(
            env::signer_account_id() == token.owner_id,
            "Only owner of the Token can set price"
        );
        assert!(
            env::attached_deposit() == OTHER_FEE,
            "Don't have enought gas fee"
        );
        token.price = price;
        token.status = TokenStatus::ForSale;
        self.search_token_data.insert(&token_id, &token);
    }

    #[payable]
    pub fn purchase(&mut self, token_id: TokenId) {
        let mut token = self.get_token_data(token_id);
        let owner_id = token.owner_id.clone();
        let token_price: Balance = token.price.clone().into();
        assert!(
            token.status == TokenStatus::ForSale,
            "Token is not for sale"
        );
        assert!(env::attached_deposit() == token_price, "Not enough balance");

        let owner_balance = self.owner_balance.get(&owner_id).unwrap_or(U128(0));
        let mut owner_balance: Balance = owner_balance.into();
        owner_balance += token_price;
        self.owner_balance.insert(&owner_id, &U128(owner_balance));
        token.status = TokenStatus::NotForSale;
        self.search_token_data.insert(&token_id, &token);

        self.transfer(env::signer_account_id(), token_id);
    }

    #[payable]
    pub fn transfer_token(&mut self, receiver_id: AccountId, token_id: TokenId) {
        assert!(
            self.get_token_data(token_id).owner_id == env::signer_account_id(),
            "Only owner of the token can call this method"
        );
        self.is_valid_token_id(token_id);
        self.transfer(receiver_id, token_id);
    }

    #[payable]
    pub fn withdraw(&mut self) {
        assert!(
            env::attached_deposit() == OTHER_FEE,
            "You don't have enough fee"
        );
        let owner_id = env::signer_account_id();
        let owner_balance = self.owner_balance.get(&owner_id).unwrap_or(U128(0));
        let owner_balance: Balance = owner_balance.into();
        assert!(owner_balance > 0, "You don't have any balance to withdraw");
        Promise::new(owner_id.clone()).transfer(owner_balance);
        self.owner_balance.insert(&owner_id, &U128(0));
    }
    //_____________________VIEW FUCTION_____________________
    pub fn get_token_data(&self, token_id: TokenId) -> TokenData {
        let token_data = self
            .search_token_data
            .get(&token_id)
            .expect("Token ID NOT FOUND!");
        token_data
    }
    pub fn get_balance(&self, owner_id: AccountId) -> U128 {
        self.owner_balance.get(&owner_id).unwrap_or(U128(0))
    }
    pub fn get_listing(&self, owner_id: AccountId) -> Vec<TokenId> {
        self.nft_listing.get(&owner_id).unwrap_or(Vec::new())
    }
    pub fn get_market_listing(&self) -> Vec<TokenData> {
        let mut result: Vec<TokenData> = Vec::new();
        for i in 1..=self.last_token_id {
            let data = self.get_token_data(i);
            if data.status == TokenStatus::ForSale {
                result.push(data);
            }
        }
        result
    }

    //_____________________PRIVATE FUNCTION_____________________
    fn mint(&mut self, owner_id: AccountId, metadata: TokenMetadata) -> TokenId {
        self.last_token_id += 1;
        let token_id = self.last_token_id;
        let token_data = TokenData {
            owner_id: owner_id.clone(),
            status: TokenStatus::NotForSale,
            price: U128(0),
            metadata,
            token_id,
        };

        let mut owner_listing = self.nft_listing.get(&owner_id).unwrap_or(Vec::new());
        owner_listing.push(self.last_token_id);
        self.search_token_data
            .insert(&self.last_token_id, &token_data);
        self.nft_listing.insert(&owner_id, &owner_listing);
        self.last_token_id
    }
    fn transfer(&mut self, receiver: AccountId, token_id: TokenId) {
        let owner_id = self.get_token_data(token_id).owner_id;

        //REMOVE THE TOKENID IN OLD OWNER ACCOUNT
        let mut owner_listing = self.nft_listing.get(&owner_id).unwrap();
        let mut pos: usize = 0;
        for i in 0..owner_listing.len() {
            if owner_listing[i] == token_id {
                pos = i;
                break;
            }
        }
        owner_listing.remove(pos);
        self.nft_listing.insert(&owner_id, &owner_listing);

        //ADD TOKENID TO THE RECEIVER ACCOUNT
        let mut receiver_listing = self.nft_listing.get(&receiver).unwrap_or(Vec::new());
        receiver_listing.push(token_id);
        self.nft_listing.insert(&receiver, &receiver_listing);

        //UPDATE THE NFT OWNER DATA
        let mut token = self.search_token_data.get(&token_id).unwrap();
        token.owner_id = receiver;
        self.search_token_data.insert(&token_id, &token);
    }

    fn is_valid_token_id(&self, token_id: TokenId) {
        assert!(token_id <= self.last_token_id, "INVALID TOKEN ID");
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;
    // use near_sdk::MockedBlockchain;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn get_metadata() -> TokenMetadata {
        "urltesting".to_string()
    }

    #[test]
    fn test_mint_token() {
        let mut context = get_context(accounts(0));
        let mut contract = Contract::new();
        testing_env!(context.attached_deposit(MINT_FEE).build());
        for _ in 0..4 {
            contract.mint_token(accounts(0), get_metadata());
        }

        testing_env!(context
            .attached_deposit(MINT_FEE)
            .signer_account_id(accounts(1))
            .build());
        for _ in 0..4 {
            contract.mint_token(accounts(1), get_metadata());
        }
        let acc0_listing = contract.get_listing(accounts(0));
        let acc1_listing = contract.get_listing(accounts(1));

        assert_eq!(acc0_listing, vec![1, 2, 3, 4]);
        assert_eq!(acc1_listing, vec![5, 6, 7, 8]);
    }

    #[test]
    fn test_transfer_function() {
        let mut context = get_context(accounts(0));
        let mut contract = Contract::new();
        testing_env!(context.attached_deposit(MINT_FEE).build());
        for _ in 0..3 {
            contract.mint_token(accounts(0), get_metadata());
        }

        contract.transfer_token(accounts(1), 1);
        contract.transfer_token(accounts(1), 2);
        let owner_listing = contract.get_listing(accounts(0));
        let receiver_listing = contract.get_listing(accounts(1));
        assert_eq!(owner_listing, vec![3]);
        assert_eq!(receiver_listing, vec![1, 2]);
    }

    #[test]
    fn test_purchase_token() {
        let mut context = get_context(accounts(0));
        testing_env!(context.attached_deposit(MINT_FEE).build());
        let mut contract = Contract::new();
        //mint_token
        for _ in 0..3 {
            contract.mint_token(accounts(0), get_metadata());
        }

        //acc0 transfer token {1, 2} to acc1
        const TEN_NEAR: Balance = 100_000_000_000_000_000_000_000_000;
        contract.transfer_token(accounts(1), 1);
        contract.transfer_token(accounts(1), 2);
        testing_env!(context
            .signer_account_id(accounts(1))
            .attached_deposit(OTHER_FEE)
            .build());
        contract.set_price(1, U128(TEN_NEAR));

        //acc0 buys token from acc1
        testing_env!(context
            .signer_account_id(accounts(0))
            .attached_deposit(TEN_NEAR)
            .build());
        contract.purchase(1);

        let acc0_listing = contract.get_listing(accounts(0));
        let acc1_listing = contract.get_listing(accounts(1));
        assert_eq!(acc0_listing, vec![3, 1]);
        assert_eq!(acc1_listing, vec![2]);

        // test balance
        let acc0_balance = contract.get_balance(accounts(0));
        let acc1_balance = contract.get_balance(accounts(1));
        assert_eq!(acc0_balance, U128(0));
        assert_eq!(acc1_balance, U128(TEN_NEAR));
    }
}
