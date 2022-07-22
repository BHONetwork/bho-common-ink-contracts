use openbrush::contracts::psp22::extensions::burnable::*;
use openbrush::traits::{AccountId, Balance};

#[openbrush::wrapper]
pub type BHC22BurnalbeRef = dyn BHC22Burnalbe;

#[openbrush::trait_definition]
pub trait BHC22Burnalbe {
    #[ink(message)]
    fn burn(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error>;
}
