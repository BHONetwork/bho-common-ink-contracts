#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
extern crate alloc;

#[openbrush::contract]
pub mod bho_psp34 {
    use ink_lang::codegen::{EmitEvent, Env};
    use ink_prelude::{string::String, vec::Vec};
    use ink_storage::traits::SpreadAllocate;
    use openbrush::contracts::psp34::extensions::{burnable::*, metadata::*, mintable::*};

    pub trait BPSP34Internal {
        /// Emits transfer event.
        fn _b_emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _id: Id, _metadata: Vec<u8>);
    }

    #[derive(Default, SpreadAllocate, PSP34Storage, PSP34MetadataStorage)]
    #[ink(storage)]
    pub struct BPSP34 {
        #[PSP34StorageField]
        psp34: PSP34Data,
        next_id: u8,
        #[PSP34MetadataStorageField]
        psp_metadata: PSP34MetadataData,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        id: Id,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        id: Option<Id>,
        approved: bool,
    }

    #[ink(event)]
    pub struct AttributeSet {
        #[ink(topic)]
        id: Id,
        key: Vec<u8>,
        data: Vec<u8>,
    }

    impl PSP34 for BPSP34 {}

    impl PSP34Metadata for BPSP34 {}

    impl PSP34Mintable for BPSP34 {}

    impl PSP34Burnable for BPSP34 {}

    impl PSP34Internal for BPSP34 {
        fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id) {
            self.env().emit_event(Transfer { from, to, id })
        }

        fn _emit_approval_event(&self, from: AccountId, to: AccountId, id: Option<Id>, approved: bool) {
            self.env().emit_event(Approval { from, to, id, approved })
        }

        fn _emit_attribute_set_event(&self, id: Id, key: Vec<u8>, data: Vec<u8>) {
            self.env().emit_event(AttributeSet { id, key, data })
        }
    }

    impl BPSP34 {
        #[ink(constructor)]
        pub fn new(metadata: Vec<u8>) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                let collection_id = instance.collection_id();
                let data: Vec<u8> = String::from("metadata").into_bytes();
                instance._set_attribute(collection_id, data, metadata);
            })
        }

        #[ink(message)]
        pub fn mint(&mut self, account: AccountId, metadata: Vec<u8>) -> Result<(), PSP34Error> {
            let id: Id = Id::U8(self.next_id);
            self._mint_to(account, id.clone())?;
            let data: Vec<u8> = String::from("metadata").into_bytes();
            self._set_attribute(id.clone(), data, metadata.clone());
            self.next_id += 1;
            Ok(())
        }
    }
}
