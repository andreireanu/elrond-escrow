#![no_std]
elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod storage;
mod escrow_format;
use escrow_format::EscrowFormat;
use escrow_format::EscrowWalletFormat;

/// Escrow any two EDSTs
#[elrond_wasm::contract]
pub trait Escrow : 
    crate::storage::StorageModule 
    {
    #[init]
    fn init(&self, start_timestamp: u64) {
        self.start_timestamp().set(start_timestamp);
    }

    #[endpoint(addOffer)]
    #[payable("*")]    
    fn add_offer(&self, token_send: TokenIdentifier, amount_send: BigUint,
        token_receive: TokenIdentifier, amount_receive: BigUint, pair_wallet: ManagedAddress ) {
        
        // Check for timestamp
        let current_timestamp = self.blockchain().get_block_timestamp();
        require!(
            current_timestamp > self.start_timestamp().get(),
            "Escrow service is not available at the moment!"
        );

        // Check for duplicate offers
        let data_own_pov: EscrowFormat<Self::Api> = EscrowFormat {
            token_send: token_send.clone(),
            amount_send: amount_send.clone(),
            token_receive: token_receive.clone(),
            amount_receive: amount_receive.clone(),
            };
        let caller = self.blockchain().get_caller();
        require!(
            self.check_duplicate_offers(&data_own_pov, &caller),
            "Error, this offer already exists!"
        );

        require!(
            caller.clone() != pair_wallet,
            "Initiator wallet is the same as the concluder wallet!"
        );

        // Check for disallowed tokens
        require!(
            !self.dissalowed_tokens().contains(&token_send),
            "The token you want to Swap From is disallowed for escrow!");

        require!(
            !self.dissalowed_tokens().contains(&token_receive),
            "The token you want to Swap To is disallowed for escrow!");

        let data_pair_pov: EscrowFormat<Self::Api> = EscrowFormat {
            token_send: token_receive.clone(),
            amount_send: amount_receive.clone(),
            token_receive: token_send.clone(),
            amount_receive: amount_send.clone(),
            };


        // Main code for adding offer
        let send_mapper_option: Option<UnorderedSetMapper<EscrowWalletFormat<Self::Api>>> = self.send_data().get(&caller); 
        match send_mapper_option {
            Some(mut send_mapper) => {
                send_mapper.insert(EscrowWalletFormat {
                    wallet: pair_wallet.clone(),
                    data: data_own_pov.clone() 
                });
            },
            None => {
                self.send_data().insert_default(caller.clone());
                self.send_data().get(&caller).unwrap().insert(EscrowWalletFormat {
                    wallet: pair_wallet.clone(),
                    data: data_own_pov.clone()
                });
            },
        };

        let receive_mapper_option: Option<UnorderedSetMapper<EscrowWalletFormat<Self::Api>>> = self.receive_data().get(&pair_wallet); 
        match receive_mapper_option {
            Some(mut receive_mapper) => {
                receive_mapper.insert(EscrowWalletFormat {
                    wallet: caller,
                    data: data_pair_pov, 
                });
            },
            None => {
                self.receive_data().insert_default(pair_wallet.clone());
                self.receive_data().get(&pair_wallet).unwrap().insert(EscrowWalletFormat {
                    wallet: caller,
                    data: data_pair_pov, 
                });
            },
        };
    }

    #[inline]
    fn check_duplicate_offers(&self, data_own_pov: &EscrowFormat<Self::Api>, caller: &ManagedAddress<Self::Api> ) -> bool {
        let send_mapper_option: Option<UnorderedSetMapper<EscrowWalletFormat<Self::Api>>> = self.send_data().get(&caller); 
        match send_mapper_option {
            Some(send_mapper) => {
                for record in send_mapper.iter() {
                    if record.data == *data_own_pov {
                        return false;
                    }
                }
                true
            },
            None => {
                true
            }
        }
    }

    #[endpoint(removeOffer)]
    fn remove_offer(&self, token_send: TokenIdentifier, amount_send: BigUint,
        token_receive: TokenIdentifier, amount_receive: BigUint, pair_wallet: ManagedAddress ) {

        let data_own_pov: EscrowFormat<Self::Api> = EscrowFormat {
            token_send: token_send.clone(),
            amount_send: amount_send.clone(),
            token_receive: token_receive.clone(),
            amount_receive: amount_receive.clone(),
            };

        let data_pair_pov: EscrowFormat<Self::Api> = EscrowFormat {
            token_send: token_receive.clone(),
            amount_send: amount_receive.clone(),
            token_receive: token_send.clone(),
            amount_receive: amount_send.clone(),
            };

        let caller = self.blockchain().get_caller();
        let send_mapper_option: Option<UnorderedSetMapper<EscrowWalletFormat<Self::Api>>> = self.send_data().get(&caller);
            
        match send_mapper_option {
            Some(mut send_mapper) => {
                for record in send_mapper.iter() {
                    if record.data == data_own_pov {
                        send_mapper.swap_remove(&record);
                        if send_mapper.len() == 0 {
                            self.receive_data().remove(&caller);
                        }
                        break;
                    }
                }
            },
            None => {},
        };

        let receive_mapper_option: Option<UnorderedSetMapper<EscrowWalletFormat<Self::Api>>> = self.receive_data().get(&pair_wallet);
        match receive_mapper_option {
            Some(mut receive_mapper) => {
                for record in receive_mapper.iter() {
                    if record.data == data_pair_pov {
                        receive_mapper.swap_remove(&record);
                        if receive_mapper.len() == 0 {
                            self.receive_data().remove(&caller);
                        }
                        self.send().direct_esdt(
                            &caller,
                            &token_send,
                            0,
                            &amount_send,
                            b"Removed offer"
                        );
                        break;
                    }
                }
            },
            None => {},
        };
    }


    #[endpoint(acceptOffer)]
    #[payable("*")]   
    fn accept_offer(&self, token_send: TokenIdentifier, amount_send: BigUint,
        token_receive: TokenIdentifier, amount_receive: BigUint, pair_wallet: ManagedAddress ) {

        let data_own_pov: EscrowFormat<Self::Api> = EscrowFormat {
            token_send: token_send.clone(),
            amount_send: amount_send.clone(),
            token_receive: token_receive.clone(),
            amount_receive: amount_receive.clone(),
            };

        let data_pair_pov: EscrowFormat<Self::Api> = EscrowFormat {
            token_send: token_receive.clone(),
            amount_send: amount_receive.clone(),
            token_receive: token_send.clone(),
            amount_receive: amount_send.clone(),
            };

        let caller = self.blockchain().get_caller();
        let receive_mapper_option: Option<UnorderedSetMapper<EscrowWalletFormat<Self::Api>>> = self.receive_data().get(&caller);
        let mut found = false;

        match receive_mapper_option {
            Some(mut receive_mapper) => {
                'outer_for: for record in receive_mapper.iter() {
                    if record.data == data_own_pov {
                        self.send().direct_esdt(
                            &pair_wallet,
                            &token_send,
                            0,
                            &amount_send,
                            b"Initiator tokens sent"
                        );
                        self.send().direct_esdt(
                            &self.blockchain().get_caller(),
                            &token_receive,
                            0,
                            &amount_receive,
                            b"Concluder tokens sent"
                        );
                        self.escrows_no().update(|value| *value += 1);
                        receive_mapper.swap_remove(&record);
                        if receive_mapper.len() == 0 {
                            self.receive_data().remove(&caller);
                        }
                        let send_mapper_option: Option<UnorderedSetMapper<EscrowWalletFormat<Self::Api>>> = self.send_data().get(&pair_wallet);
                        match send_mapper_option {
                            Some(mut send_mapper) => {
                                'inner_for: for record in send_mapper.iter() {
                                    if record.data == data_pair_pov {
                                        send_mapper.swap_remove(&record);
                                        if send_mapper.len() == 0 {
                                            self.send_data().remove(&pair_wallet);
                                        }
                                        break 'inner_for;
                                    }
                                }
                            },
                            None => {}
                        };
                        found = true;
                        break 'outer_for;
                    }
                }
                if !found{
                    require!(
                        false,
                        "Wallet hasn't made the offer you accepted!"
                    );
                };
            },
            None =>  {
                require!(
                    false,
                    "Offer from wallet inexistent!"
                );
            }
        }
    }


    // CLEAR CONTRACT

    #[only_owner]
    #[endpoint(clear)]
    fn clear(&self, address: &ManagedAddress) {
        self.send_data().remove(address);
        self.receive_data().remove(address);
    }
 
}
