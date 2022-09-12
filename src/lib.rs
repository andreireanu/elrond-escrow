#![no_std]

mod escrow_format;

use escrow_format::EscrowFormat;
use escrow_format::EscrowWalletFormat;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();
 

/// Escrow any two EDSTs
#[elrond_wasm::contract]
pub trait Escrow {
    #[init]
    fn init(&self, start_timestamp: u64) {
        self.start_timestamp().set(start_timestamp);
    }

    #[endpoint(addOffer)]
    #[payable("*")]    
    fn add_offer(&self, token_send: TokenIdentifier, amount_send: BigUint,
        token_receive: TokenIdentifier, amount_receive: BigUint, pair_wallet: ManagedAddress ) {
        
        let mut passed_timestamp = true;
        let mut passed_duplicate = true;
        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();
        if current_timestamp < self.start_timestamp().get(){
            self.send().direct_esdt(
                &caller,
                &token_send,
                0,
                &amount_send,
                b"Refunded because escrow service is not available at the moment!"
            );
            passed_timestamp = false;
        };

        if passed_timestamp {
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

            let send_mapper_option: Option<UnorderedSetMapper<EscrowWalletFormat<Self::Api>>> = self.send_data().get(&caller); 
            match send_mapper_option {
                Some(mut send_mapper_save) => {
                    for record in send_mapper_save.iter() {
                        if record.data == data_own_pov {
                            self.send().direct_esdt(
                                &caller,
                                &token_send,
                                0,
                                &amount_send,
                                b"Refunded because this offer already exists!"
                            );
                            passed_duplicate = false;
                            break;
                        }
                    }
                    if passed_duplicate {
                    send_mapper_save.insert(EscrowWalletFormat {
                        wallet: pair_wallet.clone(),
                        data: data_own_pov.clone() 
                    });
                    }
                },
                None => {
                    if passed_duplicate {
                    self.send_data().insert_default(caller.clone());
                    self.send_data().get(&caller).unwrap().insert(EscrowWalletFormat {
                        wallet: pair_wallet.clone(),
                        data: data_own_pov.clone()
                    });
                }},
            };

            if passed_duplicate {
                let receive_mapper_save_option: Option<UnorderedSetMapper<EscrowWalletFormat<Self::Api>>> = self.receive_data().get(&pair_wallet); 
                match receive_mapper_save_option {
                    Some(mut receive_mapper_save) => {
                        receive_mapper_save.insert(EscrowWalletFormat {
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
                            b"Canceled offer"
                        );
                        break;
                    }
                }
            },
            None => {},
        };
    }

    // CLEAR

    #[only_owner]
    #[endpoint(clear)]
    fn clear(&self, address: &ManagedAddress) {
        self.send_data().remove(address);
        self.receive_data().remove(address);
    }

 
    // STORAGE

    /// START TIMESTAMP
    #[storage_mapper("esdt_mapper")]
    fn start_timestamp(&self) -> SingleValueMapper<u64>;

    #[only_owner]
    #[endpoint(setStartTimestamp)]
    fn set_start_timestamp(&self, start_timestamp: u64) {
        self.start_timestamp().set(&start_timestamp)
    }

    #[only_owner]
    #[endpoint(getStartTimestamp)]
    fn get_start_timestamp(&self) {
        self.start_timestamp();
        }

    /// ALLOWED ESTDs
    #[storage_mapper("esdt_mapper")]
    fn esdt_mapper(&self) -> SetMapper<TokenIdentifier>;

    #[only_owner]
    #[endpoint(addEsdt)]
    fn add_esdt(&self, token_id: TokenIdentifier) {
        self.esdt_mapper().insert(token_id);
        }

    #[view(getEsdts)]
    fn get_esdts(&self) -> SetMapper<TokenIdentifier>{
            self.esdt_mapper()
        }
    
    /// ESCROW DATA

    /// DATA STORED WITH SENDER WALLET KEY
    #[storage_mapper("send_data")]
    fn send_data(&self) -> MapStorageMapper<ManagedAddress,  UnorderedSetMapper<EscrowWalletFormat<Self::Api>>>;

    #[view(getSendData)]
    fn get_send_data(&self, address: &ManagedAddress) ->  UnorderedSetMapper<EscrowWalletFormat<Self::Api>> 
    {
        self.send_data().get(address).unwrap()
    }

    /// DATA STORED WITH RECEIVER WALLET KEY
    #[storage_mapper("receive_data")]
    fn receive_data(&self) -> MapStorageMapper<ManagedAddress,  UnorderedSetMapper<EscrowWalletFormat<Self::Api>>>;
    
    #[view(getReceiveData)]
    fn get_receive_data_address(&self, address: &ManagedAddress ) ->  UnorderedSetMapper<EscrowWalletFormat<Self::Api>> 
    {
        self.receive_data().get(address).unwrap()
    }

 
}
