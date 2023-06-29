use soroban_sdk::{Address, Env, contractmeta, contracterror};

contractmeta!(
    key = "Description",
    val = "Constant product AMM with a .3% swap fee"
);

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 0,
    NotInitialized = 1,
    InvalidAmount = 2,
    InNotSent = 3,
    InvalidCallback = 4,
}

pub trait RPCiege5SwapCallbackTrait {

    fn swap_callback(
        env:Env,
        liqpool:Address,
        token_id:Address,
        amount:i128,
        initiator:Option<Address>
    );

}