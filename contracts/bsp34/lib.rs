#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
extern crate alloc;

#[openbrush::contract]
pub mod bsp34_contract {
    use bho_common::traits::bsp34::*;
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_prelude::vec::Vec;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::contracts::psp34::extensions::{
        burnable::*,
        metadata::*,
    };

    #[derive(Default, SpreadAllocate, PSP34Storage, PSP34MetadataStorage)]
    #[ink(storage)]
    pub struct BSP34Contract {
        #[PSP34StorageField]
        psp34: PSP34Data,
        next_id: u128,
        #[PSP34MetadataStorageField]
        psp_metadata: PSP34MetadataData,
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        id: Id,
    }

    /// Event emitted when a token approve occurs.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        operator: AccountId,
        #[ink(topic)]
        id: Option<Id>,
        approved: bool,
    }

    /// Event emitted when an attribute is set for a token.
    #[ink(event)]
    pub struct AttributeSet {
        id: Id,
        key: Vec<u8>,
        data: Vec<u8>,
    }

    impl PSP34 for BSP34Contract {}

    impl PSP34Metadata for BSP34Contract {}

    impl PSP34Internal for BSP34Contract {
        fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id) {
            self.env().emit_event(Transfer { from, to, id })
        }

        fn _emit_approval_event(&self, owner: AccountId, operator: AccountId, id: Option<Id>, approved: bool) {
            self.env().emit_event(Approval {
                owner,
                operator,
                id,
                approved,
            })
        }

        fn _emit_attribute_set_event(&self, id: Id, key: Vec<u8>, data: Vec<u8>) {
            self.env().emit_event(AttributeSet { id, key, data })
        }
    }

    impl BSP34 for BSP34Contract {
        #[ink(message)]
        fn mint(&mut self, account: AccountId, attrs: Vec<(Vec<u8>, Vec<u8>)>) -> Result<Id, PSP34Error> {
            let id: Id = Id::U128(self.next_id);
            self._mint_to(account, id.clone())?;
            for (key, data) in attrs {
                self._set_attribute(id.clone(), key, data);
            }
            self.next_id += 1;
            Ok(id)
        }

        #[ink(message)]
        fn burn(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
            self._burn_from(account, id)
        }
    }

    impl BSP34Contract {
        #[ink(constructor)]
        pub fn new(attrs: Vec<(Vec<u8>, Vec<u8>)>) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                let collection_id = instance.collection_id();
                for (key, data) in attrs {
                    instance._set_attribute(collection_id.clone(), key, data);
                }
            })
        }
    }
}
