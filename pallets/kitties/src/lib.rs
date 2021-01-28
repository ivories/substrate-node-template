#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::{decl_module,decl_storage, decl_event, decl_error, StorageValue, ensure, StorageMap, Parameter,
					traits::{Currency,Randomness,Get, ReservableCurrency}
};
use frame_system::{ensure_signed};
use sp_io::hashing::blake2_128;
use sp_std::prelude::*;
use sp_runtime::{DispatchError,traits::{AtLeast32Bit,Bounded}};

type KittyIndex = u32;

#[derive(Encode, Decode)]
pub struct Kitty(pub [u8; 16]);

pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type Randomness: Randomness<Self::Hash>;
}

decl_storage! {
    trait Store for Module<T: Trait> as Kitties {
        pub Kitties get(fn kitties): map hasher(blake2_128_concat) KittyIndex => Option<Kitty>;
        pub KittiesCount get(fn kitties_count): KittyIndex;
        pub KittyOwners get(fn kitty_owner): map hasher(blake2_128_concat) KittyIndex => Option<T::AccountId>;
    }
}

decl_error! {
    pub enum Error for Module<T: Trait> {

    }
}

decl_event!(
    pub enum Event<T> where <T as frame_system::Trait>::AccountId, {

    }
);

decl_module!{ 
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

    }
}

impl<T: Trait> Module<T> {

}

#[cfg(test)]
mod tests{
    use super::*;
}