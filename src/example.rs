#![cfg_attr(not(any(test, feature = "std")), no_std)]

use ink_core::{
    env::println,
    memory::format,
    storage,
};
use ink_lang::contract;

contract! {
    /// This simple dummy contract has a `bool` value that can
    /// alter between `true` and `false` using the `flip` message.
    /// Users can retrieve its current state using the `get` message.
    struct Mpt {
        /// The current state of our flag.
        value: storage::Value<bool>,
    }

    impl Deploy for Mpt {
        /// Initializes our state to `false` upon deploying our smart contract.
        fn deploy(&mut self) {
            self.value.set(false)
        }
    }

    impl Mpt {
        /// Flips the current state of our smart contract.
        pub(external) fn flip(&mut self) {
            *self.value = !*self.value;
        }

        /// Returns the current state.
        pub(external) fn get(&self) -> bool {
            println(&format!("Storage Value: {:?}", *self.value));
            *self.value
        }
    }
}

#[cfg(all(test, feature = "test-env"))]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut contract = Mpt::deploy_mock();
        assert_eq!(contract.get(), false);
        contract.flip();
        assert_eq!(contract.get(), true);
    }
}

/*
 *
 *#![no_std]

// Import to interact with contract storage
use ink_core::storage;
// Import the `contract!` macro
use ink_core::contract;

// The code for your contract will live entirely in the `contract!` macro
contract! {
    struct ContractName {
        my_bool: storage::Value<bool>,
    }

    impl Deploy for ContractName {
        fn deploy(&mut self) {
            // Deployment logic that runs once upon contract creation 
        }
    }

    impl ContractName {
        // Public/Private Function Definitions 
    }
}
 *
 */
