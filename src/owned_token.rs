use odra::{
    types::{Address, U256, OdraError},
    ContractEnv,
};

use crate::{erc20::{self, Erc20}, ownable::Ownable};

#[odra::module]
pub struct OwnedToken {
    ownable: Ownable,
    erc20: Erc20,
}

#[odra::module]
impl OwnedToken {
    #[odra(init)]
    pub fn init(&self, name: String, symbol: String, decimals: u8, initial_supply: U256) {
        let deployer = ContractEnv::caller();
        self.ownable.init(deployer);
        self.erc20.init(name.clone(), symbol, decimals, initial_supply);
    }
        
    pub fn name(&self) -> String {
        ContractEnv::revert(erc20::Error::InsufficientAllowance);
        self.erc20.name()
    }

    pub fn symbol(&self) -> String {
        self.erc20.symbol()
    }

    pub fn decimals(&self) -> u8 {
        self.erc20.decimals()
    }

    pub fn total_supply(&self) -> U256 {
        self.erc20.total_supply()
    }

    pub fn balance_of(&self, address: Address) -> U256 {
        self.erc20.balance_of(address)
    }

    pub fn allowance(&self, owner: Address, spender: Address) -> U256 {
        self.erc20.allowance(owner, spender)
    }

    pub fn transfer(&self, recipient: Address, amount: U256) {
        self.erc20.transfer(recipient, amount);
    }

    pub fn transfer_from(&self, owner: Address, recipient: Address, amount: U256) {
        self.erc20.transfer_from(owner, recipient, amount);
    }

    pub fn approve(&self, spender: Address, amount: U256) {
        self.erc20.approve(spender, amount);
    }

    pub fn get_owner(&self) -> Address {
        self.ownable.get_owner()
    }

    pub fn change_ownership(&self, new_owner: Address) {
        self.ownable.ensure_ownership(ContractEnv::caller());
        self.ownable.change_ownership(new_owner);
    }

    pub fn mint(&self, address: Address, amount: U256) {
        self.ownable.ensure_ownership(ContractEnv::caller());
        self.erc20.mint(address, amount);
    }

}

#[cfg(test)]
mod tests {
    use odra::{assert_events, types::U256, TestEnv};
    use super::*;

    const NAME: &str = "Plascoin";
    const SYMBOL: &str = "PLS";
    const DECIMALS: u8 = 10;
    const INITIAL_SUPPLY: u32 = 10_000;

    fn setup() -> OwnedTokenRef {
        OwnedToken::deploy_init(
            NAME.to_string(),
            SYMBOL.to_string(),
            DECIMALS,
            INITIAL_SUPPLY.into(),
        )
    }

    #[test]
    fn init_works() {
        let token = setup();
        // let owner = TestEnv::get_account(0);
        TestEnv::assert_exception(
            erc20::Error::InsufficientAllowance,
            || { let _ = token.name(); }
        );
        // assert_eq!(&token.name(), NAME);
        // assert_eq!(&token.symbol(), SYMBOL);
        // assert_eq!(token.decimals(), DECIMALS);
        // assert_eq!(token.total_supply(), INITIAL_SUPPLY.into());
        // assert_events!(
        //     token,
        //     Transfer {
        //         from: None,
        //         to: Some(owner),
        //         amount: INITIAL_SUPPLY.into()
        //     },
        //     OwnershipChanged {
        //         prev_owner: None,
        //         new_owner: owner
        //     }
        // );
    }

    // #[test]
    // fn mint_works() {
    //     let token = setup();
    //     let recipient = TestEnv::get_account(1);
    //     let amount = 10.into();
    //     token.mint(recipient, amount);
    //     assert_eq!(token.total_supply(), U256::from(INITIAL_SUPPLY) + amount);
    //     assert_eq!(token.balance_of(recipient), amount);
    // }

    // #[test]
    // fn mint_error() {
    //     let token = setup();
    //     let recipient = TestEnv::get_account(1);
    //     let amount = 10.into();
    //     TestEnv::set_caller(&recipient);
    //     TestEnv::assert_exception(
    //         ownable::Error::NotOwner,
    //         || token.mint(recipient, amount)
    //     );
    // }

    // #[test]
    // fn change_ownership_works() {
    //     let token = setup();
    //     let new_owner = TestEnv::get_account(1);
    //     token.change_ownership(new_owner);
    //     assert_eq!(token.get_owner(), new_owner);
    // }

    // #[test]
    // fn change_ownership_error() {
    //     let token = setup();
    //     let new_owner = TestEnv::get_account(1);
    //     TestEnv::set_caller(&new_owner);
    //     TestEnv::assert_exception(
    //         ownable::Error::NotOwner,
    //         || token.change_ownership(new_owner)
    //     );
    // }
}
