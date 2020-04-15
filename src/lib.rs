#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
use frame_support::sp_runtime::offchain::http;
use frame_support::sp_std::prelude::Vec;
use frame_support::{debug, decl_error, decl_event, decl_module, decl_storage, dispatch};
// use system::ensure_signed;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Encode, Decode, Default, Clone, PartialEq, Debug)]
pub struct IPFS {
    pub command: Vec<u8>,
}

/// The pallet's configuration trait.
pub trait Trait: system::Trait {

    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as TemplateModule {

        Commands get(fn commands):
            map hasher(blake2_128_concat) u32 => IPFS;
    }
}

// The pallet's events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        /// dummy event
        /// TODO: change to a valid event  
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

        pub fn ipfs_command(_origin, command: Vec<u8>) -> dispatch::DispatchResult {

            // ---------------------------------------------
            // TODO: get ipfs command
            debug::warn!("The Command is: {:?}", command);

            let ipfs_command = "http://localhost:5001/api/v0/pin/add?arg=QmV9tSDx9UiPeWExXEeH6aoDvmihvx6jD5eLb4jbTaKGps";

            // TODO: 42??
            Commands::insert(42, IPFS { command: ipfs_command.as_bytes().to_vec() });
            // ---------------------------------------------

            Ok(())
        }

        fn offchain_worker(_block: T::BlockNumber) {

            // ---------------------------------------------
            // TODO: get command from storage if any
            // TODO: remove executed command from storage

            // TODO: 42??
            let ipfs_command: IPFS = Commands::get(42);
            debug::warn!(">>>>>>>>>>>>>> The Command is: {:?}", ipfs_command);
            // ---------------------------------------------

            match Self::ipfs_run(ipfs_command) {
                Ok(_res) => debug::info!("Success!"),
                Err(e) => debug::error!("Error ipfs_run: {}", e),
            };
        }
    }
}

impl<T: Trait> Module<T> {
    fn ipfs_run(ipfs_command: IPFS) -> Result<(), &'static str> {
        // TODO: ??
        let body: Vec<&'static [u8]> = Vec::new();

        // TODO: unwrap
        let ipfs_command = frame_support::sp_std::str::from_utf8(&ipfs_command.command).unwrap();

        let pending = http::Request::post(ipfs_command, body)
            .send()
            .map_err(|_| "Error in sending http request")?;

        let response = pending
            .wait()
            .map_err(|_| "Error in waiting http response back")?;

        if response.code != 200 {
            debug::warn!("Unexpected status code: {}", response.code);
            return Err("Non-200 status code returned from http request");
        }

        Ok(())
    }
}
