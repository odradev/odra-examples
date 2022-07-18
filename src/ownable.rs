use odra::{
    types::{event::Event, Address, OdraError},
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
            ContractEnv::revert(Error::OwnerIsAleadyInitialzed)
        }
        self.owner.set(owner);
        OwnershipChanged {
            prev_owner: None,
            new_owner: owner,
        }
        .emit();
    }

    pub fn change_ownership(&self, new_owner: Address) {
        let current_owner = self.get_owner();
        if ContractEnv::caller() != current_owner {
            ContractEnv::revert(Error::NotOwner)
        }
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
            None => ContractEnv::revert(Error::OnwerIsNotInitialized)
        }
    }
}

pub enum Error {
    NotOwner,
    OwnerIsAleadyInitialzed,
    OnwerIsNotInitialized,
}

impl Into<OdraError> for Error {
    fn into(self) -> OdraError {
        match self {
            Error::NotOwner => OdraError::execution_err(3, "Not an owner"),
            Error::OnwerIsNotInitialized => OdraError::execution_err(4, "Owner is not initialized."),
            Error::OwnerIsAleadyInitialzed => OdraError::execution_err(5, "Owner is already initialized."),
        }
    }
}

#[derive(Event, Debug, PartialEq)]
pub struct OwnershipChanged {
    pub prev_owner: Option<Address>,
    pub new_owner: Address,
}

#[cfg(test)]
mod tests {
    use odra::{assert_events, TestEnv, types::VmError};
    use super::*;

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
    fn second_initialization_fails() {
        let (owner, ownable) = setup();
        TestEnv::assert_exception(OdraError::VmError(VmError::InvalidContext), || {
            ownable.init(owner);
        });
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
        let (owner, ownable) = setup();
        let new_owner = TestEnv::get_account(1);
        ownable.change_ownership(new_owner);
        TestEnv::assert_exception(Error::NotOwner, || {
            ownable.change_ownership(new_owner);
        });
    }
}
