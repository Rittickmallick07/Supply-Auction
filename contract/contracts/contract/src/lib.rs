#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Env, Symbol, Address, Map,
};

#[contract]
pub struct SupplyAuction;

// ✅ IMPORTANT: Add contracttype
#[derive(Clone)]
#[contracttype]
pub struct Auction {
    pub owner: Address,
    pub item: Symbol,
    pub highest_bid: i128,
    pub highest_bidder: Address,
    pub active: bool,
}

#[contractimpl]
impl SupplyAuction {

    // Create auction
    pub fn create_auction(env: Env, owner: Address, item: Symbol) {
        let key = symbol_short!("AUC");

        let mut auctions: Map<u32, Auction> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        let id = auctions.len();

        let auction = Auction {
            owner: owner.clone(),
            item,
            highest_bid: 0,
            highest_bidder: owner.clone(),
            active: true,
        };

        auctions.set(id, auction);
        env.storage().instance().set(&key, &auctions);
    }

    // Place bid
    pub fn place_bid(env: Env, id: u32, bidder: Address, amount: i128) {
        let key = symbol_short!("AUC");

        let mut auctions: Map<u32, Auction> =
            env.storage().instance().get(&key).unwrap();

        let mut auction = auctions.get(id).unwrap();

        if !auction.active {
            panic!("Auction not active");
        }

        if amount <= auction.highest_bid {
            panic!("Bid too low");
        }

        auction.highest_bid = amount;
        auction.highest_bidder = bidder;

        auctions.set(id, auction);
        env.storage().instance().set(&key, &auctions);
    }

    // End auction
    pub fn end_auction(env: Env, id: u32, owner: Address) {
        let key = symbol_short!("AUC");

        let mut auctions: Map<u32, Auction> =
            env.storage().instance().get(&key).unwrap();

        let mut auction = auctions.get(id).unwrap();

        if auction.owner != owner {
            panic!("Not authorized");
        }

        auction.active = false;

        auctions.set(id, auction);
        env.storage().instance().set(&key, &auctions);
    }

    // Get auction
    pub fn get_auction(env: Env, id: u32) -> Auction {
        let key = symbol_short!("AUC");

        let auctions: Map<u32, Auction> =
            env.storage().instance().get(&key).unwrap();

        auctions.get(id).unwrap()
    }
}