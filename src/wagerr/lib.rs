#![cfg_attr(not(feature = "std"), no_std, no_main)]


#[ink::contract]
mod wagerr {
    use ink::prelude::string::ToString;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
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
        status: WagerStatus,
        claimed: bool,
        claimant: Option<AccountId>,
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
        WagerNotFound,
        WagerClaimError,
        WagerActionError,
        WagerTransferError
    }

    // Events
    #[ink(event)]
    pub struct WagerCreated {
        #[ink(topic)]
        wager: Wager,
    }

    #[ink(event)]
    pub struct WagerActive {
        #[ink(topic)]
        wager: Wager,
    }

    #[ink(event)]
    pub struct WagerClaimed {
        #[ink(topic)]
        wager: Wager,
    }

    #[ink(event)]
    pub struct WagerClaimAccepted {
        #[ink(topic)]
        wager: Wager,
    }

    #[ink(event)]
    pub struct WagerClaimRejected {
        #[ink(topic)]
        wager: Wager,
    }

    #[ink(event)]
    pub struct WagerCompleted {
        #[ink(topic)]
        wager: Wager,
    }

    impl Wager {
        fn new(id: String, creator: AccountId, name: String, terms: String, amount: Balance) -> Self {
            Self {
                id,
                creator,
                name,
                terms,
                amount,
                bettor: None,
                claimant: None,
                claimed: false,
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
                    (wager.creator == account || wager.bettor == Some(account) ) && wager.status == WagerStatus::Active }
                )
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
            let int_id: u32 = (self.wagers_vec.len() + 1000).try_into().unwrap();
            let id = int_id.to_string();

            // assert the value sent >= 2
            assert!(
                transfered_amount >= minimum_amount,
                "Wager must be at least 2 units"
            );

            let wager = Wager::new(id, caller, name, terms, transfered_amount);
            self.wagers.insert(wager.id.clone(), &wager.clone());
            self.wagers_vec.push(wager.clone());

            // emit wager created event
            self.env().emit_event(WagerCreated {
                wager: wager.clone(),
            });

            wager.id
        }

        #[ink(message, payable)]
        pub fn join_wager(&mut self, id: String) -> Result<Wager, WagerrError>{
            let caller = Self::env().caller();
            let mut wager = self.get_wager(id.clone());

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

                // emit wager active event
                self.env().emit_event(WagerActive {
                    wager: wager.clone(),
                });

                Ok(wager)

            } else {
                return Err(WagerrError::WagerActive);
            }
        }
    
        #[ink(message)]
        pub fn claim_win(&mut self, id: String) -> Result<Wager, WagerrError>{
            let caller = Self::env().caller();
            let mut wager = self.get_wager(id.clone());

            // verify the caller is a bettor in the wager
            if caller != wager.creator && Some(caller) != wager.bettor {
                ink::env::debug_println!(
                    "Caller not a creator nor bettor!"
                );
                return Err(WagerrError::WagerClaimError);
            }

            // assert wager is active
            assert!(
                wager.status == WagerStatus::Active,
                "Wager is not active."
            );

            wager.claimed = true;
            wager.claimant = Some(caller);

            let index = self
                .wagers_vec
                .iter()
                .position(|wager| wager.id == id)
                .ok_or(WagerrError::WagerNotFound)?;

            self.wagers.insert(wager.id.clone(), &wager.clone());
            self.wagers_vec[index] = wager.clone();
            
            ink::env::debug_println!(
                "Claimed wager"
            );

            // send claim created event
            self.env().emit_event(WagerClaimed {
                wager: wager.clone(),
            });

            Ok(wager)

        }

        #[ink(message)]
        pub fn accept_reject_claim(&mut self, id: String, action: ClaimAction) -> Result<Wager, WagerrError> {
            let caller = Self::env().caller();
            let mut wager = self.get_wager(id.clone());

             // verify the caller is a bettor in the wager
             if caller != wager.creator && Some(caller) != wager.bettor {
                ink::env::debug_println!(
                    "Caller not a creator nor bettor!"
                );
                return Err(WagerrError::WagerActionError);
            }

            // verify there is a claim
            assert!(
                wager.claimed == true,
                "There is no claim yet."
            );

            // verify caller is not the claimant
            assert!(
                wager.claimant.expect("REASON") != caller,
                "Claimant can't accept/reject a claim."
            );

            if action == ClaimAction::Accept {
                // transfer to claimant

                self.env().emit_event(WagerClaimAccepted {
                    wager: wager.clone(),
                });
                match self
                .env()
                .transfer(wager.claimant.expect("REASON"), wager.total_stake)
                {
                    Ok(_) => {
                        // set wager to completed
                        wager.status = WagerStatus::Completed;

                        // Push to storage
                        let index = self
                            .wagers_vec
                            .iter()
                            .position(|wager| wager.id == id)
                            .ok_or(WagerrError::WagerNotFound)?;

                        self.wagers.insert(wager.id.clone(), &wager.clone());
                        self.wagers_vec[index] = wager.clone();
                        

                        // send wager complete event
                        self.env().emit_event(WagerCompleted {
                            wager: wager.clone(),
                        });
                        Ok(wager)
                    }
                    Err(_) => Err(WagerrError::WagerTransferError),
                }

                
            } else {
                // remove claimant and claim
                wager.claimed = false;
                wager.claimant = None;

                 // Push to storage
                let index = self
                 .wagers_vec
                 .iter()
                 .position(|wager| wager.id == id)
                 .ok_or(WagerrError::WagerNotFound)?;

                self.wagers.insert(wager.id.clone(), &wager.clone());
                self.wagers_vec[index] = wager.clone();

                // send claim rejected event
                self.env().emit_event(WagerClaimRejected {
                    wager: wager.clone(),
                });

                Ok(wager)
            }
        }
    }

}