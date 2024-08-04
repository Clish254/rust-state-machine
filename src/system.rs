use num::traits::{CheckedAdd, CheckedSub, One, Zero};
use std::collections::BTreeMap;

pub trait Config {
	type AccountId: Ord + Clone;
	type BlockNumber: Zero + CheckedSub + CheckedAdd + Copy + One;
	type Nonce: Zero + CheckedSub + CheckedAdd + Copy + One;
}
/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// The current block number.
	block_number: T::BlockNumber,
	/// A map from an account to their nonce.
	nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) -> crate::support::DispatchResult {
		let incremented =
			self.block_number.checked_add(&T::BlockNumber::one()).ok_or("Overflow")?;
		self.block_number = incremented;
		Ok(())
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &T::AccountId) -> crate::support::DispatchResult {
		let user_nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
		let new_nonce = user_nonce.checked_add(&T::Nonce::one()).ok_or("Overflow")?;
		self.nonce.insert(who.to_owned(), new_nonce);
		Ok(())
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;
	impl super::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}
	#[test]
	fn init_system() {
		let mut system = super::Pallet::<TestConfig>::new();
		let _ = system.inc_block_number();
		let _ = system.inc_nonce(&"alice".to_string());

		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce.get("alice"), Some(&1));
		assert_eq!(system.nonce.get("bob"), None);
	}
}
