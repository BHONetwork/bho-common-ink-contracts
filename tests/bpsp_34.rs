#![feature(min_specialization)]
#[openbrush::contract]
mod bho_psp34 {
    use ink_lang as ink;
    use ink_lang::codegen::{EmitEvent, Env};
    use ink_prelude::{string::String, vec::Vec};
    use ink_storage::traits::SpreadAllocate;
    use openbrush::contracts::psp34::extensions::{burnable::*, metadata::*, mintable::*};
    use openbrush::test_utils::accounts;

    pub trait BPSP34Internal {
        /// Emits transfer event.
        fn _b_emit_transfer_event(
            &self,
            _from: Option<AccountId>,
            _to: Option<AccountId>,
            _id: Id,
            _metadata: Vec<u8>,
        );
    }

    #[derive(Default, SpreadAllocate, PSP34Storage, PSP34MetadataStorage)]
    #[ink(storage)]
    pub struct BPSP34 {
        #[PSP34StorageField]
        psp34: PSP34Data,
        next_id: u8,
        metadata: Option<Vec<u8>>,
        #[PSP34MetadataStorageField]
        psp_metadata: PSP34MetadataData,
    }

    #[ink(event)]
    pub struct MintToken {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        id: Id,
        metadata: Vec<u8>,
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

        fn _emit_approval_event(
            &self,
            from: AccountId,
            to: AccountId,
            id: Option<Id>,
            approved: bool,
        ) {
            self.env().emit_event(Approval {
                from,
                to,
                id,
                approved,
            })
        }

        fn _emit_attribute_set_event(&self, id: Id, key: Vec<u8>, data: Vec<u8>) {
            self.env().emit_event(AttributeSet { id, key, data })
        }

        fn _do_safe_transfer_check(
            &mut self,
            _operator: &AccountId,
            _from: &AccountId,
            _to: &AccountId,
            _id: &Id,
            _data: &Vec<u8>,
        ) -> Result<(), PSP34Error> {
            Ok(())
        }
    }

    impl BPSP34 {
        #[ink(constructor)]
        pub fn new(metadata: Vec<u8>) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                let collection_id = instance.collection_id();
                let data: Vec<u8> = String::from("metadata").into_bytes();
                instance._set_attribute(collection_id, data, metadata);
                //instance.metadata = Some(metadata)
            })
        }

        #[ink(message)]
        pub fn metadata(&self) -> Option<Vec<u8>> {
            let collection_id = self.collection_id();
            let data: Vec<u8> = String::from("metadata").into_bytes();

            //self.metadata.clone()
            self.get_attribute(collection_id, data)
        }

        #[ink(message)]
        pub fn mint(&mut self, metadata: Vec<u8>) -> Result<(), PSP34Error> {
            let id: Id = Id::U8(self.next_id);
            self._mint_to(Self::env().caller(), id.clone())?;
            let data: Vec<u8> = String::from("metadata").into_bytes();
            self._set_attribute(id.clone(), data, metadata.clone());
            self.next_id += 1;
            // self._b_emit_transfer_event(None, Some(Self::env().caller()), id, metadata);
            Ok(())
        }
    }

    #[ink::test]
    fn transfer_works() {
        let accounts = accounts();
        let metadata: Vec<u8> = String::from("hello bpsp34").into_bytes();
        // Create a new contract
        let mut bpsp34 = BPSP34::new(metadata);
        // The first AttributeSet event takes place
        assert_eq!(1, ink_env::test::recorded_events().count());

        // Create token Id 1 for Alice
        assert!(bpsp34._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        // Alice owns token 1
        assert_eq!(bpsp34.balance_of(accounts.alice), 1);
        // Bob does not owns any token
        assert_eq!(bpsp34.balance_of(accounts.bob), 0);
        // The second Transfer event takes place
        assert_eq!(2, ink_env::test::recorded_events().count());
        // Alice transfer token 1 to Bob
        assert!(bpsp34.transfer(accounts.bob, Id::U8(1u8), vec![]).is_ok());
    }
}
