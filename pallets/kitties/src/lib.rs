#![cfg_attr(not(feature = "std"), no_std)]

// 第一题: 视频中的 transfer 中没有判断 kitty 是不是 owner .需要加入 ensure!(owner == from.clone(), Error::<T>::RequireOwner)


use codec::{Encode, Decode};
use frame_support::{decl_module, decl_storage, decl_event, decl_error, ensure, fail,
	StorageValue, StorageMap, Parameter,
	traits::{Randomness, Currency, ExistenceRequirement::AllowDeath, ReservableCurrency}
};
use sp_io::hashing::blake2_128;
use frame_system::ensure_signed;
use sp_runtime::{DispatchError, traits::{AtLeast32Bit, Bounded}};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


#[derive(Encode, Decode)]
pub struct Kitty(pub [u8; 16]);

/// The pallet's configuration trait.
pub trait Trait: frame_system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	type Randomness: Randomness<Self::Hash>;
	type KittyIndex: Parameter + AtLeast32Bit + Bounded + Default + Copy;
	type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
}

// type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

// This pallet's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as KittiesModule {
		pub Kitties get(fn kitties): map hasher(blake2_128_concat) T::KittyIndex => Option<Kitty>;
		pub KittiesCount get(fn kitties_count): T::KittyIndex;
		pub KittyOwners get(fn kitty_owners): map hasher(blake2_128_concat) T::KittyIndex => Option<T::AccountId>;
		pub OwnedKitties get(fn owned_kitties): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) T::KittyIndex => T::KittyIndex;
		pub KittyParents get(fn kitty_parents): map hasher(blake2_128_concat) T::KittyIndex => (T::KittyIndex, T::KittyIndex);
		pub KittyChildren get(fn kitty_children): double_map hasher(blake2_128_concat) T::KittyIndex, hasher(blake2_128_concat) T::KittyIndex => T::KittyIndex;
		pub KittySpouse get(fn kitty_spouse): double_map hasher(blake2_128_concat) T::KittyIndex, hasher(blake2_128_concat) T::KittyIndex => T::KittyIndex;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where
		AccountId = <T as frame_system::Trait>::AccountId,
		KittyIndex = <T as Trait>::KittyIndex,
	{
		Created(AccountId, KittyIndex),
		Transferred(AccountId, AccountId, KittyIndex),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		KittiesCountOverflow,
		InvalidKittyId,
		KittyNotExists,
		RequireOwner,
		TransferToSelf,
		RequireDifferentParent,
	}
}

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing errors
		type Error = Error<T>;

		// Initializing events
		fn deposit_event() = default;

		/// Create a new kitty
		#[weight = 0]
		pub fn create(origin) {
			let sender = ensure_signed(origin)?;
			let kitty_id = Self::next_kitty_id()?;

			T::Currency::reserve(&sender, 1000.into())?;

			// Generate a random 128bit value
			let dna = Self::random_value(&sender);

			// Create and store kitty
			let kitty = Kitty(dna);
			Self::insert_kitty(&sender, kitty_id, kitty, 0.into(), 0.into());

			Self::deposit_event(RawEvent::Created(sender, kitty_id));
		}

		/// Breed kitties
		#[weight = 0]
		pub fn breed(origin, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) {
			let sender = ensure_signed(origin)?;

			let new_kitty_id = Self::do_breed(&sender, kitty_id_1, kitty_id_2)?;

			Self::deposit_event(RawEvent::Created(sender, new_kitty_id));
		}

		/// Transfer a kitty to new owner
		#[weight = 0]
		pub fn transfer(origin, to: T::AccountId, kitty_id: T::KittyIndex) {
			let sender = ensure_signed(origin)?;

			ensure!(to != sender, Error::<T>::TransferToSelf);

			Self::do_transfer(&sender, &to, kitty_id)?;

			Self::deposit_event(RawEvent::Transferred(sender, to, kitty_id));
		}

	}
}

fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
	(selector & dna1) | (!selector & dna2)
}

impl<T: Trait> Module<T> {
	fn random_value(sender: &T::AccountId) -> [u8; 16] {
		let payload = (
			T::Randomness::random_seed(),
			&sender,
			<frame_system::Module<T>>::extrinsic_index(),
		);
		payload.using_encoded(blake2_128)
	}

	fn next_kitty_id() -> sp_std::result::Result<T::KittyIndex, DispatchError> {
		let kitty_id = Self::kitties_count();
		if kitty_id == T::KittyIndex::max_value() {
			return Err(Error::<T>::KittiesCountOverflow.into());
		}
		Ok(kitty_id)
	}

    fn insert_kitty(owner: &T::AccountId, kitty_id: T::KittyIndex, kitty: Kitty, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) {
        <Kitties<T>>::insert(kitty_id, kitty);
        <KittiesCount<T>>::put(kitty_id + 1.into());
        <KittyParents<T>>::insert(kitty_id, (kitty_id_1, kitty_id_2));
        if kitty_id_1 != kitty_id_2 {
			// breed
            <KittySpouse<T>>::insert(kitty_id_1, kitty_id_2, kitty_id_2);
            <KittySpouse<T>>::insert(kitty_id_2, kitty_id_1, kitty_id_1);
            <KittyChildren<T>>::insert(kitty_id_1, kitty_id, kitty_id);
            <KittyChildren<T>>::insert(kitty_id_2, kitty_id, kitty_id);
        }
        <OwnedKitties<T>>::insert(owner.clone(), kitty_id, kitty_id);
        <KittyOwners<T>>::insert(kitty_id, owner);
    }

	fn do_breed(sender: &T::AccountId, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) -> sp_std::result::Result<T::KittyIndex, DispatchError> {
		let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
		let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

		ensure!(kitty_id_1 != kitty_id_2, Error::<T>::RequireDifferentParent);
		ensure!(<OwnedKitties<T>>::contains_key(&sender, kitty_id_1), Error::<T>::RequireOwner);
		ensure!(<OwnedKitties<T>>::contains_key(&sender, kitty_id_2), Error::<T>::RequireOwner);

		let kitty_id = Self::next_kitty_id()?;

		T::Currency::reserve(&sender, 1000.into())?;

		let kitty1_dna = kitty1.0;
		let kitty2_dna = kitty2.0;

		// Generate a random 128bit value
		let selector = Self::random_value(&sender);
		let mut new_dna = [0u8; 16];

		// Combine parents and selector to create new kitty
		for i in 0..kitty1_dna.len() {
			new_dna[i] = combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i]);
		}

		Self::insert_kitty(&sender, kitty_id, Kitty(new_dna), kitty_id_1, kitty_id_2);

		Ok(kitty_id)
	}

	fn do_transfer(from: &T::AccountId, to: &T::AccountId, kitty_id: T::KittyIndex) -> sp_std::result::Result<(), DispatchError> {
		match KittyOwners::<T>::get(&kitty_id) {
            Some(owner) => ensure!(owner == from.clone(), Error::<T>::RequireOwner),
            None => fail!(Error::<T>::InvalidKittyId)
        }

		T::Currency::unreserve(&from, 1000.into());
		T::Currency::transfer(&from, &to, 1000.into(), AllowDeath)?;

        <OwnedKitties<T>>::remove(&from, kitty_id);
        <OwnedKitties<T>>::insert(to.clone(), kitty_id, kitty_id);
		<KittyOwners<T>>::insert(kitty_id, to.clone());
		
		Ok(())
	}
}