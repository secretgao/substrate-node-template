#![cfg_attr(not(feature="std"), no_std)]
pub use pallet::*;
#[frame_support::pallet]
pub mod pallet{
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;
	#[pallet::config]
	pub trait Config: frame_system::Config{
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn proofs)]
	pub type Proofs<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		Vec<u8>,
		(T::AccountId,T::BlockNumber)
	>;

	#[pallet::event]
	#[pallet::metadata(T::AccountId="AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T:Config>{
		ClaimCreated(T::AccountId,Vec<u8>),
		ClaimRemoved(T::AccountId,Vec<u8>),
		ClaimCall(T::AccountId,Vec<u8>),
	}
	#[pallet::error]
	pub enum Error<T>{
		ProofAlreadyExists,
		ClaimNotExists,
		NotClaimOwner,
		NoneValue,
		StorageOverflow,
	}
	#[pallet::hooks]
	impl<T:Config> Hooks<BlockNumberFor<T>> for Pallet<T>{}

	#[pallet::call]
	impl <T:Config> Pallet<T>{
		//创建存证 👇👇👇👇👇👇👇👇👇👇
		#[pallet::weight(0)]
		pub fn create_claim(
			origin: OriginFor<T>,//交易的发送方
			claim: Vec<u8>//存证的hash值
		)->DispatchResultWithPostInfo{
			let sender = ensure_signed(origin)?;
			ensure!(!Proofs::<T>::contains_key(&claim),Error::<T>::ProofAlreadyExists);

			Proofs::<T>::insert(
				&claim,(sender.clone(),frame_system::Pallet::<T>::block_number())
			);
			//触发事件
			Self::deposit_event(Event::ClaimCreated(sender,claim));
			Ok(().into())
		}
		//创建存证 👆👆👆👆👆👆👆👆👆👆
		//删除存证👇👇👇👇👇👇👇👇👇👇
		#[pallet::weight(0)]
		pub fn remote_claim(
			origin:OriginFor<T>, //交易的发送方
			claim:Vec<u8> //存证的hash值
		)->DispatchResultWithPostInfo{
			let sender = ensure_signed(origin)?;
			let (owner,_) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExists)?;
			ensure!(owner == sender,Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);
			Self::deposit_event(Event::ClaimRemoved(sender,claim));
			Ok(().into())
		}
		//删除存证👆👆👆👆👆👆👆👆👆👆
		//转移存证 👇👇👇👇👇👇👇👇👇👇
		#[pallet::weight(0)]
		pub fn call_claim(origin:OriginFor<T>, //交易的发送方
						  claim:Vec<u8> //存证的hash值
		)->DispatchResultWithPostInfo{
			let sender = ensure_signed(origin)?;
			let (owner,_) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExists)?;
			ensure!(owner == sender,Error::<T>::NotClaimOwner);
			//这里没写完
			//Proofs::<T>::mutate(&claim,);

			Self::deposit_event(Event::ClaimCall(sender,claim));
			Ok(().into())
		}
		//转移存证 👆👆👆👆👆👆👆👆👆👆
	}
}
