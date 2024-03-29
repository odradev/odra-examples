use odra::{
    execution_error,
    types::{event::Event, Address},
    ContractEnv, Event, Variable,
};

#[odra::module]
pub struct Ownable {
    owner: Variable<Address>,
}

#[odra::module]
impl Ownable {
    #[odra(init)]
    pub fn init(&self, owner: Address) {
        if self.owner.get().is_some() {
            ContractEnv::revert(Error::OwnerIsAlreadyInitialized)
        }
        self.owner.set(owner);
        OwnershipChanged {
            prev_owner: None,
            new_owner: owner,
        }
        .emit();
    }

    pub fn change_ownership(&self, new_owner: Address) {
        self.ensure_ownership(ContractEnv::caller());
        let current_owner = self.get_owner();
        self.owner.set(new_owner);
        OwnershipChanged {
            prev_owner: Some(current_owner),
            new_owner,
        }
        .emit();
    }

    pub fn ensure_ownership(&self, address: Address) {
        if Some(address) != self.owner.get() {
            ContractEnv::revert(Error::NotOwner)
        }
    }

    pub fn get_owner(&self) -> Address {
        match self.owner.get() {
            Some(owner) => owner,
            None => ContractEnv::revert(Error::OwnerIsNotInitialized),
        }
    }
}

execution_error! {
    pub enum Error {
        NotOwner => 3,
        OwnerIsAlreadyInitialized => 4,
        OwnerIsNotInitialized => 5,
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct OwnershipChanged {
    pub prev_owner: Option<Address>,
    pub new_owner: Address,
}

#[cfg(test)]
mod tests {
    use super::*;
    use odra::{assert_events, TestEnv};

    fn setup() -> (Address, OwnableRef) {
        let owner = TestEnv::get_account(0);
        let ownable = Ownable::deploy_init(owner);
        (owner, ownable)
    }

    #[test]
    fn initialization_works() {
        let (owner, ownable) = setup();
        assert_eq!(ownable.get_owner(), owner);
        assert_events!(
            ownable,
            OwnershipChanged {
                prev_owner: None,
                new_owner: owner
            }
        );
    }

    #[test]
    fn owner_can_change_ownership() {
        let (owner, ownable) = setup();
        let new_owner = TestEnv::get_account(1);
        TestEnv::set_caller(&owner);
        ownable.change_ownership(new_owner);
        assert_eq!(ownable.get_owner(), new_owner);
        assert_events!(
            ownable,
            OwnershipChanged {
                prev_owner: Some(owner),
                new_owner
            }
        );
    }

    #[test]
    fn non_owner_cannot_change_ownership() {
        let (_, ownable) = setup();
        let new_owner = TestEnv::get_account(1);
        ownable.change_ownership(new_owner);
        TestEnv::assert_exception(Error::NotOwner, || {
            ownable.change_ownership(new_owner);
        });
    }
}
