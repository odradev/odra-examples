use async_trait::async_trait;
use cucumber::{given, then, when, World, WorldInit};
use odra::TestEnv;
use odra_examples::erc20::{Erc20, Erc20Ref, Error};
use std::convert::Infallible;
use std::fmt::Debug;

pub const NAME: &str = "Plascoin";
pub const SYMBOL: &str = "PLS";
pub const DECIMALS: u8 = 10;
pub const INITIAL_SUPPLY: u32 = 10_000;

#[derive(WorldInit)]
pub struct Erc20World {
    contract: Option<Erc20Ref>,
}

impl Debug for Erc20World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Erc20World")
    }
}

#[async_trait(?Send)]
impl World for Erc20World {
    type Error = Infallible;
    async fn new() -> Result<Self, Infallible> {
        Ok(Self { contract: None })
    }
}

#[given(expr = "ERC20 token is deployed")]
fn erc20_token_is_deployed(world: &mut Erc20World) {
    world.contract = Some(Erc20::deploy_init(
        NAME.to_string(),
        SYMBOL.to_string(),
        DECIMALS,
        INITIAL_SUPPLY.into(),
    ));
}

#[when(expr = "I transfer {int} {word} to account {int}")]
fn i_transfer_amount_symbol_to_address(
    world: &mut Erc20World,
    amount: u32,
    _symbol: String,
    account: u32,
) {
    let contract = world.contract.as_ref().unwrap();
    let amount = amount.into();
    let recipient = TestEnv::get_account(account as usize);
    contract.transfer(recipient, amount);
}

#[then(expr = "I transfer {int} {word} to account {int} and it throws an error")]
fn i_transfer_amount_symbol_to_address_and_it_throws_an_error(
    world: &mut Erc20World,
    amount: u32,
    _symbol: String,
    account: u32,
) {
    let contract = world.contract.as_ref().unwrap();
    let amount = amount.into();
    let recipient = TestEnv::get_account(account as usize);
    TestEnv::assert_exception(Error::InsufficientBalance, || {
        contract.transfer(recipient, amount);
    });
}

#[when(expr = "I approve {int} {word} for account {int}")]
fn i_approve_amount_symbol_to_address(
    world: &mut Erc20World,
    amount: u32,
    _symbol: String,
    account: u32,
) {
    let contract = world.contract.as_ref().unwrap();
    let amount = amount.into();
    let recipient = TestEnv::get_account(account as usize);
    contract.approve(recipient, amount);
}

#[when(expr = "account {int} transfers {int} {word} from account {int} to account {int}")]
fn account_account_transfers_amount_symbol_to_address(
    world: &mut Erc20World,
    spender: u32,
    amount: u32,
    _symbol: String,
    owner: u32,
    recipient: u32,
) {
    let contract = world.contract.as_ref().unwrap();
    let amount = amount.into();
    let spender = TestEnv::get_account(spender as usize);
    let owner = TestEnv::get_account(owner as usize);
    let recipient = TestEnv::get_account(recipient as usize);

    TestEnv::set_caller(&spender);
    contract.transfer_from(owner, recipient, amount);
}

#[then(expr = "I have {int} {word}")]
fn i_have_amount_symbol(world: &mut Erc20World, amount: u32, _symbol: String) {
    let contract = world.contract.as_ref().unwrap();
    let amount = amount.into();
    assert_eq!(contract.balance_of(TestEnv::get_account(0)), amount);
}

#[then(expr = "account {int} has {int} {word}")]
fn account_has_amount_symbol(world: &mut Erc20World, account: u32, amount: u32, _symbol: String) {
    let contract = world.contract.as_ref().unwrap();
    let amount = amount.into();
    assert_eq!(
        contract.balance_of(TestEnv::get_account(account as usize)),
        amount
    );
}

#[then(expr = "{word} event is emitted")]
fn event_is_emitted(world: &mut Erc20World, event_name: String) {
    let contract = world.contract.as_ref().unwrap();
    let actual = odra::test_utils::get_event_name(&contract.address(), -1).unwrap();
    assert_eq!(actual, event_name);
}

#[then(expr = "{word} event is emitted at {int}")]
fn event_is_emitted_at(world: &mut Erc20World, event_name: String, at: i32) {
    let contract = world.contract.as_ref().unwrap();
    let actual = odra::test_utils::get_event_name(&contract.address(), at).unwrap();
    assert_eq!(actual, event_name);
}

#[then(expr = "total supply is {int}")]
fn total_supply_is(world: &mut Erc20World, expected: u32) {
    let contract = world.contract.as_ref().unwrap();
    let actual = contract.total_supply();
    assert_eq!(actual, expected.into());
}

#[then(expr = "symbol is {word}")]
fn symbol_is(world: &mut Erc20World, symbol: String) {
    let contract = world.contract.as_ref().unwrap();
    let actual = contract.symbol();
    assert_eq!(actual, symbol);
}

#[then(expr = "name is {word}")]
fn name_is(world: &mut Erc20World, name: String) {
    let contract = world.contract.as_ref().unwrap();
    let actual = contract.name();
    assert_eq!(actual, name);
}

#[then(expr = "decimals is {int}")]
fn decimals_is(world: &mut Erc20World, expected: u8) {
    let contract = world.contract.as_ref().unwrap();
    let actual = contract.decimals();
    assert_eq!(actual, expected);
}

// This runs before everything else, so you can setup things here.
fn main() {
    futures::executor::block_on(Erc20World::run("tests/features/contracts"));
}
