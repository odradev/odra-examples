use odra::{
    execution_error,
    types::{event::Event, Address, U256},
    ContractEnv, Event, Mapping, Variable,
};

#[odra::module]
pub struct Erc20 {
    decimals: Variable<u8>,
    symbol: Variable<String>,
    name: Variable<String>,
    total_supply: Variable<U256>,
    balances: Mapping<Address, U256>,
    allowances: Mapping<(Address, Address), U256>,
}

#[odra::module]
impl Erc20 {
    #[odra(init)]
    pub fn init(&self, name: String, symbol: String, decimals: u8, initial_supply: U256) {
        let caller = ContractEnv::caller();
        self.name.set(name);
        self.symbol.set(symbol);
        self.decimals.set(decimals);
        self.mint(caller, initial_supply);
    }

    pub fn transfer(&self, recipient: Address, amount: U256) {
        let caller = ContractEnv::caller();
        self.raw_transfer(caller, recipient, amount);
    }

    pub fn transfer_from(&self, owner: Address, recipient: Address, amount: U256) {
        let spender = ContractEnv::caller();
        self.spend_allowance(owner, spender, amount);
        self.raw_transfer(owner, recipient, amount);
    }

    pub fn approve(&self, spender: Address, amount: U256) {
        let owner = ContractEnv::caller();
        self.allowances.set(&(owner, spender), amount);
        Approval {
            owner,
            spender,
            value: amount,
        }
        .emit();
    }

    pub fn name(&self) -> String {
        self.name.get_or_default()
    }

    pub fn symbol(&self) -> String {
        self.symbol.get_or_default()
    }

    pub fn decimals(&self) -> u8 {
        self.decimals.get_or_default()
    }

    pub fn total_supply(&self) -> U256 {
        self.total_supply.get_or_default()
    }

    pub fn balance_of(&self, address: Address) -> U256 {
        self.balances.get_or_default(&address)
    }

    pub fn allowance(&self, owner: Address, spender: Address) -> U256 {
        self.allowances.get_or_default(&(owner, spender))
    }
}

impl Erc20 {
    fn raw_transfer(&self, owner: Address, recipient: Address, amount: U256) {
        let owner_balance = self.balances.get_or_default(&owner);
        if amount > owner_balance {
            ContractEnv::revert(Error::InsufficientBalance)
        }
        self.balances.set(&owner, owner_balance - amount);
        self.balances.add(&recipient, amount);
        Transfer {
            from: Some(owner),
            to: Some(recipient),
            amount,
        }
        .emit();
    }

    fn spend_allowance(&self, owner: Address, spender: Address, amount: U256) {
        let key = (owner, spender);
        let allowance = self.allowances.get_or_default(&key);
        if allowance < amount {
            ContractEnv::revert(Error::InsufficientAllowance)
        }
        self.allowances.set(&key, allowance - amount);
        Approval {
            owner,
            spender,
            value: allowance - amount,
        }
        .emit();
    }

    pub fn mint(&self, address: Address, amount: U256) {
        self.balances.add(&address, amount);
        self.total_supply.add(amount);
        Transfer {
            from: None,
            to: Some(address),
            amount,
        }
        .emit();
    }
}

#[derive(Event, PartialEq, Eq, Debug)]
pub struct Transfer {
    pub from: Option<Address>,
    pub to: Option<Address>,
    pub amount: U256,
}

#[derive(Event, PartialEq, Eq, Debug)]
pub struct Approval {
    pub owner: Address,
    pub spender: Address,
    pub value: U256,
}

execution_error! {
    pub enum Error {
        InsufficientBalance => 1,
        InsufficientAllowance => 2,
    }
}

#[cfg(test)]
pub mod tests {
    use super::{Approval, Erc20, Erc20Ref, Error, Transfer};
    use odra::{assert_events, types::U256, TestEnv};

    pub const NAME: &str = "Plascoin";
    pub const SYMBOL: &str = "PLS";
    pub const DECIMALS: u8 = 10;
    pub const INITIAL_SUPPLY: u32 = 10_000;

    pub fn setup() -> Erc20Ref {
        Erc20::deploy_init(
            NAME.to_string(),
            SYMBOL.to_string(),
            DECIMALS,
            INITIAL_SUPPLY.into(),
        )
    }

    #[test]
    fn initialization() {
        let erc20 = setup();

        assert_eq!(&erc20.symbol(), SYMBOL);
        assert_eq!(&erc20.name(), NAME);
        assert_eq!(erc20.decimals(), DECIMALS);
        assert_eq!(erc20.total_supply(), INITIAL_SUPPLY.into());
        assert_events!(
            erc20,
            Transfer {
                from: None,
                to: Some(TestEnv::get_account(0)),
                amount: INITIAL_SUPPLY.into()
            }
        );
    }

    #[test]
    fn transfer_works() {
        let erc20 = setup();
        let (sender, recipient) = (TestEnv::get_account(0), TestEnv::get_account(1));
        let amount = 1_000.into();

        erc20.transfer(recipient, amount);

        assert_eq!(
            erc20.balance_of(sender),
            U256::from(INITIAL_SUPPLY) - amount
        );
        assert_eq!(erc20.balance_of(recipient), amount);
        assert_events!(
            erc20,
            Transfer {
                from: Some(sender),
                to: Some(recipient),
                amount
            }
        );
    }

    #[test]
    fn transfer_error() {
        let erc20 = setup();
        let recipient = TestEnv::get_account(1);
        let amount = U256::from(INITIAL_SUPPLY) + U256::one();

        TestEnv::assert_exception(Error::InsufficientBalance, || {
            erc20.transfer(recipient, amount)
        });
    }

    #[test]
    fn transfer_from_and_approval_work() {
        let erc20 = setup();
        let (owner, recipient, spender) = (
            TestEnv::get_account(0),
            TestEnv::get_account(1),
            TestEnv::get_account(2),
        );
        let approved_amount = 3_000.into();
        let transfer_amount = 1_000.into();

        // Owner approves Spender.
        erc20.approve(spender, approved_amount);

        // Allowance was recorded.
        assert_eq!(erc20.allowance(owner, spender), approved_amount);
        assert_events!(
            erc20,
            Approval {
                owner,
                spender,
                value: approved_amount
            }
        );

        // Spender transfers tokens from Owner to Recipient.
        TestEnv::set_caller(&spender);
        erc20.transfer_from(owner, recipient, transfer_amount);

        // Tokens are transfered and allowance decremented.
        assert_eq!(
            erc20.balance_of(owner),
            U256::from(INITIAL_SUPPLY) - transfer_amount
        );
        assert_eq!(erc20.balance_of(recipient), transfer_amount);
        assert_events!(
            erc20,
            Approval {
                owner,
                spender,
                value: approved_amount - transfer_amount
            },
            Transfer {
                from: Some(owner),
                to: Some(recipient),
                amount: transfer_amount
            }
        );
    }

    #[test]
    fn transfer_from_error() {
        let erc20 = setup();
        let (owner, spender) = (TestEnv::get_account(0), TestEnv::get_account(1));
        let amount = 1_000.into();

        TestEnv::set_caller(&spender);
        TestEnv::assert_exception(Error::InsufficientAllowance, || {
            erc20.transfer_from(owner, spender, amount)
        });
    }
}
