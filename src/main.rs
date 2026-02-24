mod balances;
mod system;


pub struct Runtime {
    system: system::Pallet,
    balances: balances::Pallet,
}


impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}


fn main() {
    let mut runtime = Runtime::new();

    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(&alice, 100);
    runtime.system.inc_block_number();

    assert_eq!(runtime.system.block_number(),1);

    runtime.system.inc_nonce(&alice);

    let res = runtime.balances.transfer(alice.clone(), bob.clone(), 30).map_err(|e| println!("Error: {:?}", e));

    runtime.system.inc_nonce(&alice);

    let res = runtime.balances.transfer(alice.clone(), charlie.clone(), 20).map_err(|e| println!("Error: {:?}", e));

    
}


#[cfg(test)]
mod tests {
    use std::u128;

    use crate::balances;


#[test]
fn init_balances(){
    let mut balances = balances::Pallet::new();

    assert_eq!(balances.get_balance(&"alice".to_string()), 0);

    balances.set_balance(&"alice".to_string(), 100);

    assert_eq!(balances.get_balance(&"alice".to_string()),100);

    assert_eq!(balances.get_balance(&"bob".to_string()), 0);
}

#[test]
fn transfer_balance(){
    let mut balances = balances::Pallet::new();

    let alice: String = "alice".to_string();
    let bob: String = "bob".to_string();

    balances.set_balance(&alice, 100);
    let _ = balances.transfer(alice.clone(), bob.clone(), 10);

    assert_eq!(balances.get_balance(&alice), 90);
    assert_eq!(balances.get_balance(&bob), 10)
}

#[test]
fn transfer_balance_insufficient(){
    let mut balances = balances::Pallet::new();

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
    let mut balances = balances::Pallet::new();

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
