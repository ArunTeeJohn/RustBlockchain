use core::fmt::Debug;
use std::collections::BTreeMap;

use crate::support::DispatchResult;

pub trait Config: crate::system::Config {
    type Content: Debug + Ord;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    claims: BTreeMap<T::Content, T::AccountId>
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new()
        }
    }

    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(claim)
    }

    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult{
        match self.get_claim(&claim) {
            Some(_) => Err("Claim already exists"),
            None => {
                self.claims.insert(claim, caller);
                Ok(())
            } 
        }
    }

    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult{
        let claim_owner = self.get_claim(&claim).ok_or("Claim does not exist")?;
        if claim_owner != &caller {
            return Err("Caller is not the owner of the claim");
        }

        self.claims.remove(&claim);
        Ok(())
    }
}


// We should expect that the caller of each call will be provided by the dispatcher,
// and not included as a parameter of the call.
pub enum Call<T: Config> {
	CreateClaim {claim: T::Content},
    RevokeClaim {claim: T::Content},
}

/// Implementation of the dispatch logic, mapping from `POECall` to the appropriate underlying
/// function we want to execute.
impl<T: Config> crate::support::Dispatch for Pallet<T> {
	/*
		TODO:
		Implement `crate::support::Dispatch` for `Pallet<T>`.

		In your `dispatch` logic, match on `call` and forward the `caller` and `claim` data to the
		appropriate function.
	*/

    type Caller = T::AccountId;
    type Call = Call<T>;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
        match call {
            Call::RevokeClaim { claim } => {
                self.revoke_claim(caller, claim)
            },
            Call::CreateClaim { claim } => {
                self.create_claim(caller, claim)
            }
        }
    }
}


#[cfg(test)]
mod test {
	struct TestConfig;

	impl super::Config for TestConfig {
		type Content = &'static str;
	}

	impl crate::system::Config for TestConfig {
		type AccountId = &'static str;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn basic_proof_of_existence() {
		/*
			TODO:
			Create an end to end test verifying the basic functionality of this pallet.
				- Check the initial state is as you expect.
				- Check that all functions work successfully.
				- Check that all error conditions error as expected.
		*/
	}
}