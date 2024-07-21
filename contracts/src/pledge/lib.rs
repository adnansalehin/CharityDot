#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod pledge {
    use ink::storage::Mapping;

    #[derive(scale::Encode, scale::Decode, Default, Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Charity {
        name: Vec<u8>,
        target_balance: Balance,
        actual_balance: Balance,
    }

    #[derive(scale::Encode, scale::Decode, Default, Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Donation {
        donator: AccountId,
        amount: Balance,
        timestamp: u64,
    }

    #[derive(scale::Encode, scale::Decode, Default, Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct TransferRecord {
        amount: Balance,
        timestamp: u64,
        description: Vec<u8>,
    }

    #[ink(storage)]
    pub struct PledgeContract {
        charities: Mapping<AccountId, Charity>,
        donations: Mapping<AccountId, Vec<Donation>>,
        transfer_records: Mapping<AccountId, Vec<TransferRecord>>,
    }

    impl PledgeContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                charities: Mapping::default(),
                donations: Mapping::default(),
                transfer_records: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn register_charity(&mut self, charity_account: AccountId, name: Vec<u8>, target_balance: Balance) -> Result<(), &'static str> {
            if self.charities.contains(&charity_account) {
                return Err("Charity already registered");
            }
            let charity = Charity {
                name,
                target_balance,
                actual_balance: 0,
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
            let timestamp = self.env().block_timestamp();

            let donation = Donation {
                donator,
                amount,
                timestamp,
            };

            self.donations.entry(charity_account).or_insert_with(Vec::new).push(donation);

            let mut charity = self.charities.get(&charity_account).unwrap();
            charity.actual_balance = charity.actual_balance.checked_add(amount).ok_or("Overflow adding to actual balance")?;
            self.charities.insert(&charity_account, &charity);

            Ok(())
        }

        #[ink(message)]
        pub fn transfer_funds(&mut self, charity_account: AccountId, amount: Balance, description: Vec<u8>) -> Result<(), &'static str> {
            if !self.charities.contains(&charity_account) {
                return Err("Charity not registered");
            }

            let timestamp = self.env().block_timestamp();
            let transfer_record = TransferRecord {
                amount,
                timestamp,
                description,
            };

            self.transfer_records.entry(charity_account).or_insert_with(Vec::new).push(transfer_record);

            let mut charity = self.charities.get(&charity_account).unwrap();
            charity.actual_balance = charity.actual_balance.checked_sub(amount).ok_or("Insufficient balance for transfer")?;
            self.charities.insert(&charity_account, &charity);

            Ok(())
        }

        #[ink(message)]
        pub fn get_charity(&self, charity_account: AccountId) -> Option<Charity> {
            self.charities.get(&charity_account)
        }

        #[ink(message)]
        pub fn get_donations(&self, charity_account: AccountId) -> Vec<Donation> {
            self.donations.get(&charity_account).unwrap_or_default()
        }

        #[ink(message)]
        pub fn get_transfer_records(&self, charity_account: AccountId) -> Vec<TransferRecord> {
            self.transfer_records.get(&charity_account).unwrap_or_default()
        }
    }
}