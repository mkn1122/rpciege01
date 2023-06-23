#![no_std]

// You have been scammed! The pool will use your liquidity without giving any yield and there is no `withdraw` function to get your deposit back. To complete the challenge exploit the contract to get your funds back.
// Deployer contract code: https://gist.github.com/StellarQuest/28e522c43dc87dcd912c6057913a38c9
// Liquidity Pool contract code: https://gist.github.com/StellarQuest/b716fa112e965ffafd6bc2f9bb973463
const EVENT_MSG: [u8; 407] = [
    89, 111, 117, 32, 104, 97, 118, 101, 32, 98, 101, 101, 110, 32, 115, 99, 97, 109, 109, 101,
    100, 33, 32, 84, 104, 101, 32, 112, 111, 111, 108, 32, 119, 105, 108, 108, 32, 117, 115, 101,
    32, 121, 111, 117, 114, 32, 108, 105, 113, 117, 105, 100, 105, 116, 121, 32, 119, 105, 116,
    104, 111, 117, 116, 32, 103, 105, 118, 105, 110, 103, 32, 97, 110, 121, 32, 121, 105, 101, 108,
    100, 32, 97, 110, 100, 32, 116, 104, 101, 114, 101, 32, 105, 115, 32, 110, 111, 32, 96, 119,
    105, 116, 104, 100, 114, 97, 119, 96, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 116, 111,
    32, 103, 101, 116, 32, 121, 111, 117, 114, 32, 100, 101, 112, 111, 115, 105, 116, 32, 98, 97,
    99, 107, 46, 32, 84, 111, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 104, 101, 32, 99,
    104, 97, 108, 108, 101, 110, 103, 101, 32, 101, 120, 112, 108, 111, 105, 116, 32, 116, 104,
    101, 32, 99, 111, 110, 116, 114, 97, 99, 116, 32, 116, 111, 32, 103, 101, 116, 32, 121, 111,
    117, 114, 32, 102, 117, 110, 100, 115, 32, 98, 97, 99, 107, 46, 10, 68, 101, 112, 108, 111,
    121, 101, 114, 32, 99, 111, 110, 116, 114, 97, 99, 116, 32, 99, 111, 100, 101, 58, 32, 104,
    116, 116, 112, 115, 58, 47, 47, 103, 105, 115, 116, 46, 103, 105, 116, 104, 117, 98, 46, 99,
    111, 109, 47, 83, 116, 101, 108, 108, 97, 114, 81, 117, 101, 115, 116, 47, 50, 56, 101, 53, 50,
    50, 99, 52, 51, 100, 99, 56, 55, 100, 99, 100, 57, 49, 50, 99, 54, 48, 53, 55, 57, 49, 51, 97,
    51, 56, 99, 57, 10, 76, 105, 113, 117, 105, 100, 105, 116, 121, 32, 80, 111, 111, 108, 32, 99,
    111, 110, 116, 114, 97, 99, 116, 32, 99, 111, 100, 101, 58, 32, 104, 116, 116, 112, 115, 58,
    47, 47, 103, 105, 115, 116, 46, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 83, 116,
    101, 108, 108, 97, 114, 81, 117, 101, 115, 116, 47, 98, 55, 49, 54, 102, 97, 49, 49, 50, 101,
    57, 54, 53, 102, 102, 97, 102, 100, 54, 98, 99, 50, 102, 57, 98, 98, 57, 55, 51, 52, 54, 51,
];

// mod callback {
//     soroban_sdk::contractimport!(
//         file = "../../targe/wasm32-unknown-unknown/release/rpciege_5_callback_example.wasm"
//     );
// }

mod contract;
mod interface;
mod storage;
mod types;