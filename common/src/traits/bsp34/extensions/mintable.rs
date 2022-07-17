use ::ink_env::AccountId;
use ::ink_prelude::vec::Vec;
use ::openbrush::contracts::traits::psp34::*;

#[openbrush::wrapper]
pub type BSP34MintableRef = dyn BSP34Mintable;

#[openbrush::trait_definition]
pub trait BSP34Mintable {
    #[ink(message)]
    fn mint(&mut self, account: AccountId, attrs: Vec<(Vec<u8>, Vec<u8>)>) -> Result<Id, PSP34Error>;
}
