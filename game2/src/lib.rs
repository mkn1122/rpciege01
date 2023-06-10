#![no_std]
use soroban_sdk::{contractimpl, Address, Env, String};
pub struct Contract;
#[contractimpl]
impl Contract {
    pub fn game_2(env: Env, _nft_dest: Address) -> String {
        let s = String::from_slice(&env, "1694-1727");
        s
    }
}