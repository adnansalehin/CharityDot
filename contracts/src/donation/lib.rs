// donation smart contract
#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod donation {

    // use ink_lang as ink; 
    use ink::prelude::string::String;

    #[ink::contract]
    mod donation {
        // type Balance = u128;

        #[ink(storage)]
        pub struct Donation {
            value: Balance,
        }

        impl Donation {
            #[ink(constructor)]
            pub fn new(init_value: Balance) -> Self {
                Self { value: init_value }
            }

            #[ink(message)]
            pub fn donate(&mut self, value: Balance) {
                // fix unexpected side effect
                match self.value.checked_add(value) {
                    Some(new_value) => self.value = new_value,
                    None => {
                        // Handle the overflow case, e.g., by logging an error, returning an Err, etc.
                        // This is a placeholder; the actual handling will depend on the function's return type and error handling strategy.
                        panic!("Attempted to add with overflow");
                    },
                }
            }

            #[ink(message)]
            pub fn get(&self) -> Balance {
                self.value
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[ink::test]
            fn new_works() {
                let donation = Donation::new(100);
                assert_eq!(donation.get(), 100);
            }

            #[ink::test]
            fn donate_works() {
                let mut donation = Donation::new(100);
                donation.donate(50);
                assert_eq!(donation.get(), 150);
            }
        }
    }
}