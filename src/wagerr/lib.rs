#![cfg_attr(not(feature = "std"), no_std, no_main)]


#[ink::contract]
mod wagerr {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    // use ink::prelude::format;
    use ink::storage::Mapping;
    use scale::{Decode, Encode};


    #[ink(storage)]
    pub struct Wagerr {
        wagers: Mapping<String, Wager>,
        wagers_vec: Vec<Wager>
    }

    #[derive(Encode, Decode, Debug, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Wager {
        creator: AccountId,
        bettor: Option<AccountId>,
        id: String,
        name: String,
        terms: String,
        amount: Balance,
        total_stake: Balance,
        status: WagerStatus
    }

    #[derive(Debug, PartialEq, Eq, Encode, Decode, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum WagerStatus {
        Pending,
        Active,
        Completed,
    }

    #[derive(Debug, PartialEq, Eq, Encode, Decode, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum ClaimAction {
        Accept,
        Reject,
    }

    // For catching errors
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum WagerrError {
        /// Errors types for different errors.
        PaymentError,
        WagerActive,
        WagerJoinError,
        WagerNotFound
    }


    impl Wager {
        fn new(creator: AccountId, name: String, terms: String, amount: Balance) -> Self {
            let id = String::from("my-id");
            Self {
                id,
                creator,
                name,
                terms,
                amount,
                bettor: None,
                status: WagerStatus::Pending,
                total_stake: amount
            }       
        }

    }

    impl Wagerr {

        /// Creates a new wagerr contract
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                wagers: Mapping::new(),
                wagers_vec: Vec::new()
            }
        }

        #[ink(message)]
        pub fn get_wager(&self, id: String) -> Wager {
            // get single wager
            let wager = self.wagers.get(id).expect("Oh no!, Wager not found");

            wager
        }

        #[ink(message)]
        pub fn get_active_wagers(&self) -> Vec<Wager> {
            // get active created or joined wagers by an account
            let account =  Self::env().caller();
            self.wagers_vec
                .iter()
                .filter(|wager | { 
                    (wager.creator == account && wager.status == WagerStatus::Active) || wager.bettor == Some(account) 
                })
                .cloned()
                .collect()
        }

        #[ink(message)]
        pub fn get_pending_wagers(&self) -> Vec<Wager> {
            // get pending created wagers by an account
            let account =  Self::env().caller();
            self.wagers_vec
                .iter()
                .filter(|wager | { 
                    wager.creator == account && wager.status == WagerStatus::Pending 
                })
                .cloned()
                .collect()
        }

        #[ink(message, payable)]
        pub fn create_wager(&mut self, name: String, terms: String) -> String {
            // Gets the caller account id
            let caller = Self::env().caller();

            let minimum_amount: Balance = 2_000_000_000_000; // this equals to 2 Azeros
            let transfered_amount = self.env().transferred_value();

            // assert the value sent >= 2
            assert!(
                transfered_amount >= minimum_amount,
                "Wager must be at least 2 units"
            );

            let wager = Wager::new(caller, name, terms, transfered_amount);
            self.wagers.insert(wager.id.clone(), &wager.clone());
            self.wagers_vec.push(wager.clone());

            // TODO: emit wager created event

            wager.id
        }

        #[ink(message, payable)]
        pub fn join_wager(&mut self, id: String) -> Result<Wager, WagerrError>{
            let caller = Self::env().caller();
            let mut wager = self.wagers.get(id.clone()).ok_or(WagerrError::WagerNotFound)?;

            if caller == wager.creator {
                ink::env::debug_println!(
                    "Caller cannot be creator!"
                );
                return Err(WagerrError::WagerJoinError);
            }

            // checks if wager is still pending
            if wager.bettor.is_none() && wager.status == WagerStatus::Pending {

                let transfered_amount = self.env().transferred_value();
                assert!(
                    transfered_amount == wager.amount,
                    "Amount is incomplete."
                );

                wager.bettor = Some(caller);
                wager.status = WagerStatus::Active;
                wager.total_stake += transfered_amount;

                let index = self
                    .wagers_vec
                    .iter()
                    .position(|wager| wager.id == id)
                    .ok_or(WagerrError::WagerNotFound)?;

                self.wagers.insert(wager.id.clone(), &wager.clone());
                self.wagers_vec[index] = wager.clone();
                
                ink::env::debug_println!(
                    "Joined wager"
                );
                Ok(wager)

                // TODO: emit wager active event
            } else {
                return Err(WagerrError::WagerActive);
            }
        }
    
        pub fn claim_win(){

        }

        pub fn accept_reject_claim(&mut self, action: ClaimAction) {

        }
    }

}