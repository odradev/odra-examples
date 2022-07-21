use odra::{
    types::{Address, U256},
};

#[odra::module]
pub struct BalanceChecker {}

#[odra::module]
impl BalanceChecker {
    pub fn check_balance(&self, token: Address, account: Address) -> U256 {
        TokenRef::at(token).balance_of(account)
    }
}

#[odra::external_contract]
trait Token {
    fn balance_of(&self, account: Address) -> U256;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{erc20, ownable, Erc20Ref, Erc20, erc20::tests::*};
    use odra::TestEnv;

    #[test]
    fn balance_checker() {
        let (owner, empty_account) = (TestEnv::get_account(0), TestEnv::get_account(1));
        let balance_checker = BalanceChecker::deploy();
        TestEnv::set_caller(&owner);
        let token = setup_erc20();
        // let token = erc20::tests::setup();
        // let expected_owner_balance = erc20::tests::INITIAL_SUPPLY;

        let onwer_balance = balance_checker.check_balance(token.address(), owner);
        // let onwer_balance = token.balance_of(owner);
        // assert_eq!(onwer_balance.as_u32(), expected_owner_balance);        
    }

    pub fn setup_erc20() -> Erc20Ref {
        Erc20::deploy_init(
            NAME.to_string(),
            SYMBOL.to_string(),
            DECIMALS,
            INITIAL_SUPPLY.into(),
        )
    }

}
