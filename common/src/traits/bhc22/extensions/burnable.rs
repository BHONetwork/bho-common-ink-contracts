use openbrush::{
    contracts::psp22::extensions::burnable::*,
    traits::{
        AccountId,
        Balance,
    },
};

#[openbrush::wrapper]
pub type BHC22BurnableRef = dyn BHC22Burnable;

#[openbrush::trait_definition]
pub trait BHC22Burnable {
    #[ink(message)]
    fn burn(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error>;
}
