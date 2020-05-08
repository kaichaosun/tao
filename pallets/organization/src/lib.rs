#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::prelude::*;
use codec::{Encode, Decode};
use frame_support::{
	decl_module, decl_storage, decl_event, decl_error,
	RuntimeDebug, dispatch,
};
use frame_system::{self as system, ensure_signed};
use sp_runtime::traits::Hash;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Clone, Eq, PartialEq, Encode, Decode, Default, RuntimeDebug)]
pub struct OrganizationInfo<AccountId, BlockNumber> {
	name: Vec<u8>,
	description: Vec<u8>,
	creator: AccountId,
	created_at: BlockNumber,
	// board_change_threshold: u8,
	// member_change_threshold: u8,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Default, RuntimeDebug)]
pub struct Member<AccountId, BlockNumber> {
	account_id: AccountId,
	// title: Vec<u8>, // TODO change to role
	joined_at: BlockNumber,
	// is_board_member: bool,
}

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as OrganizationModule {
		Something get(fn something): Option<u32>;

		Organizations get(fn organizations): map hasher(blake2_128_concat) T::Hash => OrganizationInfo<T::AccountId, T::BlockNumber>;
		Participants get(fn participants): map hasher(blake2_128_concat) T::AccountId => Vec<T::Hash>;
		OrganizationMembers get(fn org_members): map hasher(blake2_128_concat) T::Hash => Vec<Member<T::AccountId, T::BlockNumber>>;
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

		/// Create an organization
		///
		/// # <weight>
		/// # </weight>
		#[weight = frame_support::weights::SimpleDispatchInfo::default()]
		pub fn create_organization(
			origin, name: Vec<u8>, description: Vec<u8>,
			members: Option<Vec<T::AccountId>>,
		) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			
			// TODO
			// check the length of name not exceed 32
			// check the org is not registered

			let block_number = system::Module::<T>::block_number();

			let org_id = (&who, &name).using_encoded(T::Hashing::hash);

			let org_info = OrganizationInfo {
				name,
				description,
				creator: who.clone(),
				created_at: block_number,
			};

			Organizations::<T>::insert(org_id, org_info);
			Participants::<T>::mutate(&who, |v| v.push(org_id));

			let creator = Member {
				account_id: who.clone(),
				joined_at: block_number,
			};
			let mut org_members: Vec<Member<T::AccountId, T::BlockNumber>> = members.unwrap_or(vec![])
				.iter()
				.map(|m| Member {
					account_id: m.clone(),
					joined_at: block_number,
				})
				.collect();
			org_members.push(creator);
			
			OrganizationMembers::<T>::insert(org_id, org_members);

			Ok(())
		}

		/// Just a dummy entry point.
		/// function that can be called by the external world as an extrinsics call
		/// takes a parameter of the type `AccountId`, stores it, and emits an event
		#[weight = frame_support::weights::SimpleDispatchInfo::default()]
		pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let who = ensure_signed(origin)?;

			// Code to execute when something calls this.
			// For example: the following line stores the passed in u32 in the storage
			Something::put(something);

			// Here we are raising the Something event
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			Ok(())
		}

		/// Another dummy entry point.
		/// takes no parameters, attempts to increment storage value, and possibly throws an error
		#[weight = frame_support::weights::SimpleDispatchInfo::default()]
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
