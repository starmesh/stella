#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, debug, dispatch};
use system::{offchain, ensure_signed};

// use primitives::crypto::KeyTypeId;
use sp_runtime::offchain::KeyTypeId;
use sp_runtime::transaction_validity::{
  TransactionValidity, TransactionLongevity, ValidTransaction, InvalidTransaction
};


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"abcd");

pub mod crypto {
  pub use super::KEY_TYPE;
  use sp_runtime::app_crypto::{app_crypto, sr25519};
  app_crypto!(sr25519, KEY_TYPE);
}

pub trait Trait: timestamp::Trait + system::Trait {
  /// The overarching event type.
  type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
  type Call: From<Call<Self>>;

  // type SubmitSignedTransaction: offchain::SubmitSignedTransaction<Self, <Self as Trait>::Call>;
  type SubmitUnsignedTransaction: offchain::SubmitUnsignedTransaction<Self, <Self as Trait>::Call>;
}

// This pallet's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {

		Something get(fn something): Option<u32>;

		Entries: map hasher(blake2_256) T::AccountId => T::Hash; 
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {

		SomethingStored(u32, AccountId),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Value was None
		NoneValue,
		/// Value reached maximum and cannot be incremented further
		StorageOverflow,
	}
}

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing errors
		// this includes information about your errors in the node's metadata.
		// it is needed only if you are using errors in your pallet
		type Error = Error<T>;

		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;

	    pub fn onchain_callback(_origin, _block: T::BlockNumber/*, input: Vec<u8>*/) -> dispatch::DispatchResult  {
	      debug::info!("{:?}", core::str::from_utf8(&input).unwrap());
	      Ok(())
	    }

	    fn offchain_worker(block: T::BlockNumber) {
	      // Here we specify the function to be called back on-chain in next block import.
	      // debug::info!("Hello World.");

	      let call = Call::onchain_callback(block/*, b"hello world!".to_vec()*/);
	      T::SubmitUnsignedTransaction::submit_unsigned(call);
	    }


		pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;

			Something::put(something);

			Self::deposit_event(RawEvent::SomethingStored(something, who));
			Ok(())
		}

		pub fn submit_hash(origin, hash: T::Hash) -> dispatch::DispatchResult{
			let sender = ensure_signed(origin)?;

			<Entries<T>>::insert(sender, hash);

			Ok(())
		}

		/// Another dummy entry point.
		/// takes no parameters, attempts to increment storage value, and possibly throws an error
		pub fn cause_error(origin) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let _who = ensure_signed(origin)?;

			match Something::get() {
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					Something::put(new);
					Ok(())
				},
			}
		}
	}
}

// #[allow(deprecated)]
// impl<T: Trait> frame_support::unsigned::ValidateUnsigned for Module<T> {
//   type Call = Call<T>;

//   fn validate_unsigned(call: &Self::Call) -> TransactionValidity {

//     match call {
//       Call::onchain_callback(block, input) => Ok(ValidTransaction {
//         priority: 0,
//         requires: vec![],
//         provides: vec![(block, input).encode()],
//         longevity: TransactionLongevity::max_value(),
//         propagate: true,
//       }),
//       _ => InvalidTransaction::Call.into()
//     }
//   }
// }

