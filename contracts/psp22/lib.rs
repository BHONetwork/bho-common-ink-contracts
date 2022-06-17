#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod psp22 {
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::contracts::psp22::{
        extensions::metadata::*,
        *,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PSP22Storage, PSP22MetadataStorage)]
    pub struct PSP22Token {
        #[PSP22StorageField]
        psp22: PSP22Data,
        #[PSP22MetadataStorageField]
        psp22_metadata: PSP22MetadataData,
    }

    impl PSP22 for PSP22Token {}

    impl PSP22Token {
        #[ink(constructor)]
        pub fn new(name: Option<String>, symbol: Option<String>, decimals: u8, initial_supply: Balance) -> Self {
            ink_lang::utils::initialize_contract(|instance: &mut Self| {
                instance.psp22_metadata.name = name;
                instance.psp22_metadata.symbol = symbol;
                instance.psp22_metadata.decimals = decimals;
                instance
                    ._mint(instance.env().caller(), initial_supply)
                    .expect("Failed to mint initial supply")
            })
        }
    }
}
