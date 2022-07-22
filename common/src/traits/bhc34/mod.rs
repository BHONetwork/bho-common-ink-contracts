use ::ink_env::AccountId;
use ::ink_prelude::vec::Vec;
use openbrush::contracts::psp34::{
    extensions::metadata::*,
    *,
};

#[openbrush::wrapper]
pub type BHC34Ref = dyn BHC34 + PSP34 + PSP34Metadata;

#[openbrush::trait_definition]
pub trait BHC34 {
    #[ink(message)]
    fn mint(&mut self, account: AccountId, attrs: Vec<(Vec<u8>, Vec<u8>)>) -> Result<Id, PSP34Error>;

    #[ink(message)]
    fn burn(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error>;
}
