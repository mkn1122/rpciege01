#![no_std]

mod contract;
mod storage;
mod types;

pub mod liqpool {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/game5_liquidity_pool.wasm"
    );
}