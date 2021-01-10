#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, traits::Get,
};
use frame_system::ensure_signed;
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: frame_system::Trait + pallet_mission_tokens::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as ValidatorRegistry {
        MissionOf get(fn mission_of): map hasher(blake2_128_concat) T::AccountId => T::MissionTokenId;
        GuardianOf get(fn guardian_of): map hasher(blake2_128_concat) T::AccountId => T::AccountId;
        ValidatorOf get(fn validator_of): map hasher(blake2_128_concat) T::AccountId => T::AccountId;
        Guardians get(fn guardians): map hasher(blake2_128_concat) T::MissionTokenId => Vec<T::AccountId>;
        Validators get(fn validators): map hasher(blake2_128_concat) T::MissionTokenId => Vec<T::AccountId>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        MissionTokenId = <T as pallet_mission_tokens::Trait>::MissionTokenId,
    {
        Registered(AccountId, MissionTokenId, AccountId),
        Unregistered(AccountId, MissionTokenId, AccountId),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        AlreadyRegistered,
        GuardianAlreadyRegistered,
        NotFound,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn register(origin, mission_token_id: T::MissionTokenId, guardian: T::AccountId) -> dispatch::DispatchResult {
            let validator = ensure_signed(origin)?;

            <pallet_mission_tokens::Module<T>>::validate_mission_token_id(mission_token_id)?;
            ensure!(!<GuardianOf<T>>::contains_key(&validator), Error::<T>::AlreadyRegistered);
            ensure!(!<MissionOf<T>>::contains_key(&validator), Error::<T>::AlreadyRegistered);
            ensure!(!<ValidatorOf<T>>::contains_key(&guardian), Error::<T>::GuardianAlreadyRegistered);

            <GuardianOf<T>>::insert(&validator, guardian.clone());
            <MissionOf<T>>::insert(&validator, mission_token_id);
            <ValidatorOf<T>>::insert(&guardian, validator.clone());
            <Guardians<T>>::mutate(mission_token_id, |guardians| {
                guardians.push(guardian.clone())
            });
            <Validators<T>>::mutate(mission_token_id, |validators| {
                validators.push(validator.clone())
            });

            Self::deposit_event(RawEvent::Registered(validator, mission_token_id, guardian));
            Ok(())
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn unregister(origin) -> dispatch::DispatchResult {
            let validator = ensure_signed(origin)?;

            ensure!(<MissionOf<T>>::contains_key(&validator), Error::<T>::NotFound);

            let mission_token_id = <MissionOf<T>>::get(&validator);
            let guardian = <GuardianOf<T>>::get(&validator);
            <GuardianOf<T>>::remove(&validator);
            <MissionOf<T>>::remove(&validator);
            <ValidatorOf<T>>::remove(&guardian);
            <Guardians<T>>::mutate(mission_token_id, |guardians| {
                guardians.retain(|account_id| account_id != &guardian)
            });
            <Validators<T>>::mutate(mission_token_id, |validators| {
                validators.retain(|account_id| account_id != &validator)
            });

            Self::deposit_event(RawEvent::Unregistered(validator, mission_token_id, guardian));
            Ok(())
        }
    }
}
