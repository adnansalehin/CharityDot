// donation smart contract
#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod donation {

    // use ink_lang as ink; 
    use ink::storage::Mapping;

    #[derive(scale::Encode, scale::Decode, Default, Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Donation {
        donator: AccountId,
        charity: AccountId,
        amount: Balance,
    }

    #[derive(scale::Encode, scale::Decode, Default, Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Charity {
        name: Vec<u8>,
        total_received: Balance,
    }

    #[ink(storage)]
    pub struct DonationContract {
        balances: Mapping<AccountId, Balance>,
        donations: Vec<Donation>,
        charities: Mapping<AccountId, Charity>,
    }

    impl DonationContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                balances: Mapping::default(),
                donations: Vec::new(),
                charities: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn register_charity(&mut self, charity_account: AccountId, name: Vec<u8>) -> Result<(), &'static str> {
            if self.charities.contains(&charity_account) {
                return Err("Charity already registered");
            }
            let charity = Charity {
                name,
                total_received: 0,
            };
            self.charities.insert(&charity_account, &charity);
            Ok(())
        }

        #[ink(message)]
        pub fn donate(&mut self, charity_account: AccountId, amount: Balance) -> Result<(), &'static str> {
            if !self.charities.contains(&charity_account) {
                return Err("Charity not registered");
            }

            let donator = self.env().caller();
            self.safe_transfer(donator, charity_account, amount)?;
            
            let mut charity = self.charities.get(&charity_account).unwrap();
            charity.total_received = charity.total_received.checked_add(amount).ok_or("Overflow adding to charity total")?;
            self.charities.insert(&charity_account, &charity);

            self.donations.push(Donation {
                donator,
                charity: charity_account,
                amount,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn deposit(&mut self, amount: Balance) -> Result<(), &'static str> {
            let caller = self.env().caller();
            self.safe_add_balance(caller, amount)
        }

        #[ink(message)]
        pub fn withdraw(&mut self, amount: Balance) -> Result<(), &'static str> {
            let caller = self.env().caller();
            self.safe_sub_balance(caller, amount)
        }

        #[ink(message)]
        pub fn get_balance(&self, owner: AccountId) -> Balance {
            self.balances.get(&owner).unwrap_or(0)
        }

        #[ink(message)]
        pub fn get_donations(&self) -> Vec<Donation> {
            self.donations.clone()
        }

        #[ink(message)]
        pub fn get_charity(&self, charity_account: AccountId) -> Option<Charity> {
            self.charities.get(&charity_account)
        }

        fn safe_add_balance(&mut self, account: AccountId, amount: Balance) -> Result<(), &'static str> {
            let current_balance = self.balances.get(&account).unwrap_or(0);
            let new_balance = current_balance.checked_add(amount).ok_or("Overflow adding balance")?;
            self.balances.insert(&account, &new_balance);
            Ok(())
        }

        fn safe_sub_balance(&mut self, account: AccountId, amount: Balance) -> Result<(), &'static str> {
            let current_balance = self.balances.get(&account).unwrap_or(0);
            let new_balance = current_balance.checked_sub(amount).ok_or("Insufficient balance")?;
            self.balances.insert(&account, &new_balance);
            Ok(())
        }

        fn safe_transfer(&mut self, from: AccountId, to: AccountId, amount: Balance) -> Result<(), &'static str> {
            self.safe_sub_balance(from, amount)?;
            self.safe_add_balance(to, amount)
        }
    }
}