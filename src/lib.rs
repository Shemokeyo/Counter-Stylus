//!
//! Stylus Hello World
//!
//! The following contract implements the Counter example from Foundry.
//!
//! ```solidity
//! contract Counter {
//!     uint256 public number;
//!     function setNumber(uint256 newNumber) public {
//!         number = newNumber;
//!     }
//!     function increment() public {
//!         number++;
//!     }
//! }
//! ```
//!
//! The program is ABI-equivalent with Solidity, which means you can call it from both Solidity and Rust.
//! To do this, run `cargo stylus export-abi`.
//!
//! Note: this code is a template-only and has not been audited.
//!
// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::U256, prelude::*};

// Define some persistent storage using the Solidity ABI.
// `Counter` will be the entrypoint.
sol_storage! {
    /// Persistent storage for the Counter contract.
    /// Contains a single field `number` of type uint256.
    #[entrypoint]
    pub struct Counter {
        uint256 number;
    }
}

/// Declare that `Counter` is a contract with the following external methods.
#[public]
impl Counter {
    /// Gets the number from storage.
    ///
    /// # Returns
    /// * `U256` - The current value of `number` in storage.
    pub fn number(&self) -> U256 {
        self.number.get()
    }

    /// Sets a number in storage to a user-specified value.
    ///
    /// # Arguments
    /// * `new_number` - The value to set `number` to.
    pub fn set_number(&mut self, new_number: U256) {
        self.number.set(new_number);
    }

    /// Multiplies the current number by `new_number` and stores the result.
    ///
    /// # Arguments
    /// * `new_number` - The value to multiply with the current `number`.
    pub fn mul_number(&mut self, new_number: U256) {
        self.number.set(new_number * self.number.get());
    }

    /// Adds `new_number` to the current number and stores the result.
    ///
    /// # Arguments
    /// * `new_number` - The value to add to the current `number`.
    pub fn add_number(&mut self, new_number: U256) {
        self.number.set(new_number + self.number.get());
    }

    /// Increments `number` by 1 and updates its value in storage.
    pub fn increment(&mut self) {
        let number = self.number.get();
        self.set_number(number + U256::from(1));
    }

    /// Adds the wei value from msg_value to the number in storage.
    ///
    /// This function is payable and can be called with a value transfer.
    /// If the value sent is less than 1, the function returns early.
    #[public]
    #[payable]
    pub fn add_from_msg_value(&mut self) {
        let value = self.vm().msg_value();
        if value < U256::from(1) {
            // Optionally revert, e.g. panic!("msg.value too low");
            return;
        }
        self.set_number(self.number.get() + value);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_counter() {
        use stylus_sdk::testing::*;
        // Create a test VM and a contract instance bound to it.
        let vm = TestVM::default();
        let mut contract = Counter::from(&vm);

        // Test initial value is zero.
        assert_eq!(U256::ZERO, contract.number());

        // Test increment.
        contract.increment();
        assert_eq!(U256::from(1), contract.number());

        // Test add_number.
        contract.add_number(U256::from(3));
        assert_eq!(U256::from(4), contract.number());

        // Test mul_number.
        contract.mul_number(U256::from(2));
        assert_eq!(U256::from(8), contract.number());

        // Test set_number.
        contract.set_number(U256::from(100));
        assert_eq!(U256::from(100), contract.number());

        // Override the msg value for future contract method invocations.
        vm.set_value(U256::from(2));

        // Test add_from_msg_value.
        contract.add_from_msg_value();
        assert_eq!(U256::from(102), contract.number());
    }
}
