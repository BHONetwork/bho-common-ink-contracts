use openbrush::contracts::psp22::extensions::mintable::*;
use openbrush::traits::{AccountId, Balance};

#[openbrush::wrapper]
pub type BHC22MintableRef = dyn BHC22Mintable;

#[openbrush::trait_definition]
pub trait BHC22Mintable {
    #[ink(message)]
    fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error>;
}
