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
    /// Initzialization of the token.
    #[odra(init)]
    pub fn init(&self, name: String, symbol: String, decimals: u8, initial_supply: U256) {
        let caller = ContractEnv::caller();
        self.name.set(name);
        self.symbol.set(symbol);
        self.decimals.set(decimals);
        self.mint(caller, initial_supply);
    }

    /// Transfers tokens tokens to the `recipient` from the `owner` balance.
    /// The `caller` needs to be to be approved to spend tokens beforehand.     
    pub fn transfer(&self, recipient: Address, amount: U256) {
        let caller = ContractEnv::caller();
        self.raw_transfer(caller, recipient, amount);
    }

    /// 'Spender' sends tokens to the 'recipient' from the 'owner' balance.
    pub fn transfer_from(&self, owner: Address, recipient: Address, amount: U256) {
        let spender = ContractEnv::caller();
        self.spend_allowance(owner, spender, amount);
        self.raw_transfer(owner, recipient, amount);
    }

    /// Thanks to this function 'spender' can use tokens of the 'owner'.
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

    /// Returns the name of the token.
    pub fn name(&self) -> String {
        self.name.get_or_default()
    }

    /// Returns the symbol of the token.
    pub fn symbol(&self) -> String {
        self.symbol.get_or_default()
    }

    /// Returns the decimals value of the token.
    pub fn decimals(&self) -> u8 {
        self.decimals.get_or_default()
    }

    /// Returns the amount of all tokens ever minted.
    pub fn total_supply(&self) -> U256 {
        self.total_supply.get_or_default()
    }

    /// Returns the amount of tokens for every user.
    pub fn balance_of(&self, address: Address) -> U256 {
        self.balances.get_or_default(&address)
    }

    /// Returns 'amount' the 'owner' allowed the 'spender' to spend the tokens.
    pub fn allowance(&self, owner: Address, spender: Address) -> U256 {
        self.allowances.get_or_default(&(owner, spender))
    }
}

impl Erc20 {
    /// Transfers tokens from 'owner' to 'recipient'. 
    /// It also checks if the 'owner' has enough tokens.
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

    /// Increments a balance of a given `address` by the `amount` of tokens.    
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

    /// Decrements the balance of the 'address' by a given 'amount' of tokens.
    pub fn burn(&self, address: Address, amount: U256) {
        self.balances.subtract(&address, amount);
        self.total_supply.subtract(amount);
        Transfer {
            from: Some(address),
            to: None,
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
