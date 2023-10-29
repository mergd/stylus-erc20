
#![cfg_attr(not(feature = "export-abi"), no_main)]

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
extern crate alloc;
use crate::erc20::{ERC20Params, ERC20};
mod erc20;


struct testERC20;
impl ERC20Params for testERC20 {
    const NAME: &'static str = "Mock ERC20";
    const SYMBOL: &'static str = "MOCK";

}


sol_storage! {
    #[entrypoint]
    pub struct Mock {
        #[borrow]
        ERC20<testERC20> erc20;
    }
}

#[external]
#[inherit(ERC20<testERC20>)]
impl Mock {
    #[payable] 
    pub fn mint(&mut self, amount:U256) => Result<(), Err> {
        self.erc20._mint(msg::sender(), amount);
        Ok(())
    }

    pub fn burn(&mut self, amount:U256) => Result<(), Err> {
        self.erc20._burn(msg::sender, amount);
        Ok(())
    }
}

impl Mock {
    // Additional 
}


