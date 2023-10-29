/// Import the Stylus SDK along with alloy primitive types for use in our program.
use alloc::{string::String, vec::Vec};
use core::marker::PhantomData;
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    alloy_sol_types::sol,
    evm, msg,
    prelude::*,
};

// Define the entrypoint as a Solidity storage object, in this case a struct
// called `Counter` with a single uint256 value called `number`. The sol_storage! macro
// will generate Rust-equivalent structs with all fields mapped to Solidity-equivalent
// storage slots and types.
sol_storage! {
   pub struct ERC20<T> {
    string name;
    string symbol;
    mapping(uint256 => uint256) balances;
    mapping(address => mapping(address => uint256)) allowances;
    uint256 totalSupply;

   }
}

sol! {
    event Transfer(address indexed from, address indexed to, uint256 indexed id);
    event Approval(address indexed owner, address indexed spender, uint256 indexed id);

    error ERC20InsufficientBalance(address sender, uint256 balance, uint256 needed);
    error ERC20InvalidSender(address sender);
    error ERC20InvalidReceiever(address receiver);
    error ERC20InsufficientAllowance(address spender, uint256 alllowance, uint256 needed);
    error ERC20InvalidApprove(address approver);
    error ERC20InvalidSpender(address spender);

}

pub trait ERC20Params {
    const NAME: &'static str;
    const SYMBOL: &'static str;
}
#[external]
impl<T: ERC20Params> ERC20<T> {
    pub fn name(&self) -> Result<String, ()> {
        Ok(T::NAME.into())
    }
    pub fn symbol(&self) -> Result<String, ()> {
        Ok(T::SYMBOL.into())
    }

    pub fn totalSupply(&self) -> Result<U256, ()> {
        Ok(self.totalSupply)
    }


    pub fn balanceOf(&self, owner: Address) -> Result<U256, ()> {
        Ok(self.balances.get(owner).unwrap_or(0u256))
    }

    pub fn allowance(&self, owner: Address, spender: Address) -> Result<U256, ()> {
        Ok(self.balances.get(owner).get(spender).unwrap_or(0u256))
    }

    pub fn transfer(&mut self, receiver: Address, amount: U256) -> Result<(), ERC20Error> {
        let caller = msg::sender();
        if self.balanceOf(caller).unwrap() > amount {
            return Err(ERC20Error::ERC20InsufficientBalance.into())
        }
        let old_bal = self.balances.get(caller);
        let mut sender_bal_setter = self.balances.setter(caller);
        sender_bal_setter.set(old_bal - amount);

        let receiver_bal = self.balances.get(receiver);
        let mut receiver_bal_setter = self.balances.setter(receiver);
        receiver_bal_setter.set(receiver_bal + amount);
        evm::log(Transfer{caller, receiver, amount});
        Ok(())

    }

    pub fn transferFrom(&mut self, sender: Address, receiver: Address, amount: U256) -> Result<(), ERC20Error> {
        let caller = msg::sender();
        if sender != caller {
            // check for approval by sender
            let approveAmt = self.allowance(caller, sender).unwrap();
            if approveAmt < amount {
                return Err(ERC20Error::ERC20InsufficientAllowance.into())
            }
            // Spend the approval amount
            let mut allowanceSetter = self.allowances.get(sender).setter(caller);
            allowanceSetter.set(approveAmt - amount)
        }
        let sender_bal = self.balanceOf(sender).unwrap();
        if sender_bal < amount {
            return Err(ERC20Error::ERC20InsufficientBalance.into())
        }

        let mut sender_bal_setter = self.balance.setter(sender);
        sender_bal_setter.set(sender_bal - amount);
        let receiver_bal = self.balanceOf(receiver).unwrap();
        let receiver_bal_setter = self.balance.setter(receiver).unwrap();
        receiver_bal_setter(receiver_bal + amount)

        evm::log(Transfer{sender, receiver, amount});
        Ok(())
    }

    pub fn approve(&mut self, spender: Address, amount: U256) -> Result<(), ()> {
        let caller = msg::sender();
        let approval_amt_setter = self.approval.setter(caller);
        approval_amt_setter.set(amount);
        Ok(())
    }
}

// internal methods

impl<T: ERC20Params> ERC20<T> {
    pub fn _mint(receiver: Address, amount: U256) -> Result<(), ()> {
        let receiver_bal = self.balances.get(receiver);
        let mut receiver_bal_setter = self.balances.setter(receiver);
        receiver_bal_setter.set(receiver_bal + amount);
        evm::log(Transfer{Address::ZERO, receiver, amount});
        Ok(())
    }
    pub fn _burn(receiver: Address, amount: U256) -> Result<(), ERC20Error> {
        let receiver_bal = self.balances.get(receiver);
        if receiver_bal < amount {
            return Err(ERC20Error::ERC20InsufficientBalance.into())
        }
        let mut receiver_bal_setter = self.balances.setter(receiver);
        receiver_bal_setter.set(receiver_bal - amount);
        evm::log(Transfer{receiver, Address::ZERO, amount});
        Ok(())
    }
}