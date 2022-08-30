use async_trait::async_trait;
use cucumber::{given, then, World, WorldInit};
use odra_examples::erc20::{Erc20, Erc20Ref};
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
    futures::executor::block_on(Erc20World::run("tests/features/book"));
}
