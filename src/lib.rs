#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::sp_runtime::offchain::http;
use frame_support::sp_std::prelude::Vec;
use frame_support::{debug, decl_error, decl_event, decl_module, decl_storage, dispatch};
// use system::ensure_signed;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: system::Trait {

    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as TemplateModule {

        Command get(fn command): Option<Vec<u8>>;
    }
}

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

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// Value was None
        NoneValue,
        /// Value reached maximum and cannot be incremented further
        StorageOverflow,
    }
}

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
            debug::warn!("[Extrinsic] The byte command is: {:?}", command);

            // TODO: real command
            let ipfs_command = "http://localhost:5001/api/v0/pin/add?arg=QmV9tSDx9UiPeWExXEeH6aoDvmihvx6jD5eLb4jbTaKGps";
            Command::put(ipfs_command.as_bytes().to_vec());

            Ok(())
        }

        fn offchain_worker(_block: T::BlockNumber) {
            if let Some(command) = Command::get(){
                debug::info!("[Offchain worker] The command is: {:?}", command);

                match Self::ipfs_run(command) {
                    Ok(_res) => {
                        debug::info!("Success!");
                        // TODO: remove executed command from storage
                        // Cmd::put(None);
                    }
                    Err(e) => debug::error!("Error ipfs_run: {}", e),
                };
            };
        }
    }
}

impl<T: Trait> Module<T> {
    fn ipfs_run(command: Vec<u8>) -> Result<(), &'static str> {
        let command = frame_support::sp_std::str::from_utf8(&command).unwrap_or_else(|_err| {
            return "Error in parsing command";
        });

        let pending = http::Request::post(command, Vec::<&'static [u8]>::new())
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
