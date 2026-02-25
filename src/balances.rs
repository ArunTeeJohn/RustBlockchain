use std::collections::BTreeMap;

use num::{CheckedAdd, CheckedSub, Zero};

pub trait Config: crate::system::Config {
    type Balance: Zero + CheckedAdd + CheckedSub + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>
}

impl <T: Config> Pallet<T>
{
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new()
        }
    }

    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance){
        self.balances.insert(who.clone(), amount);
    }

    pub fn get_balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }

    pub fn transfer(&mut self, caller: T::AccountId, to: T::AccountId, amount: T::Balance) -> Result<(),&'static str>{
        let caller_balance = self.get_balance(&caller);
        let to_balance = self.get_balance(&to);

        let new_caller_balance = caller_balance.checked_sub(&amount).ok_or("Insufficient Balance")?;
        let new_to_balance = to_balance.checked_add(&amount).ok_or("addition overflow error")?;

        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use std::u128;

    use crate::system;

    struct TestConfig;
    impl system::Config for TestConfig{
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    impl super::Config for TestConfig{
        type Balance = u128;
    }

#[test]
fn init_balances(){
    let mut balances: super::Pallet<TestConfig> = super::Pallet::new();

    assert_eq!(balances.get_balance(&"alice".to_string()), 0);

    balances.set_balance(&"alice".to_string(), 100);

    assert_eq!(balances.get_balance(&"alice".to_string()),100);

    assert_eq!(balances.get_balance(&"bob".to_string()), 0);
}

#[test]
fn transfer_balance(){
    let mut balances: super::Pallet<TestConfig> = super::Pallet::new();

    let alice: String = "alice".to_string();
    let bob: String = "bob".to_string();

    balances.set_balance(&alice, 100);
    let _ = balances.transfer(alice.clone(), bob.clone(), 10);

    assert_eq!(balances.get_balance(&alice), 90);
    assert_eq!(balances.get_balance(&bob), 10)
}

#[test]
fn transfer_balance_insufficient(){
    let mut balances: super::Pallet<TestConfig> = super::Pallet::new();

    let alice: String = "alice".to_string();
    let bob: String = "bob".to_string();

    balances.set_balance(&alice, 100);
    let result = balances.transfer(alice.clone(), bob.clone(), 110);

    assert_eq!(result,Err("Insufficient Balance"));
    assert_eq!(balances.get_balance(&alice), 100);
    assert_eq!(balances.get_balance(&bob), 0);
}

#[test]
fn transfer_balance_overflow(){
    let mut balances: super::Pallet<TestConfig> = super::Pallet::new();

    let alice: String = "alice".to_string();
    let bob: String = "bob".to_string();

    balances.set_balance(&alice, 100);
    balances.set_balance(&bob, u128::MAX);
    let result = balances.transfer(alice.clone(), bob.clone(), 10);

    assert_eq!(result,Err("addition overflow error"));
    assert_eq!(balances.get_balance(&alice), 100);
    assert_eq!(balances.get_balance(&bob), u128::MAX);
}



}
