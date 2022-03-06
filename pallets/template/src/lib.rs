#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_support::traits::Currency;
	use frame_system::pallet_prelude::*;

    // Handles our pallet's currency abstraction
    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    // Struct for holding asset information
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Asset<T: Config> {
        pub asset_id: u32,
        // `None` assumes not for sale
        pub price: Option<BalanceOf<T>>,
        pub owner: T::AccountId,
    }




	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {

		/// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The Currency handler for the kitties pallet.
        type Currency: Currency<Self::AccountId>;

        /// The maximum amount of assets a single account can own.
        #[pallet::constant]
        type MaxAssetsOwned: Get<u32>;

        /// The maximum number of assets in a blockchain
        #[pallet::constant]
        type MaxNumberOfAssets: Get<u32>;


	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
        /// An account reached maximum of assets owned
        TooManyOwned,
        /// Trying to transfer or buy an asset from oneself.
        TransferToSelf,
        /// This kitty already exists!
        DuplicateKitty,
        /// This asset already exists!
        DuplicateAsset,
        /// This kitty does not exist!
        NoKitty,
        /// This asset does not exist!
        NoAsset,
        /// You are not the owner of this asset.
        NotOwner,
        /// This asset is not for sale.
        NotForSale,
        /// Ensures that the buying price is greater than the asking price.
        BidPriceTooLow,
        /// You need to have two cats with different gender to breed.
        CantBreed,
        /// This person doesn't have enough resources to buy this land asset
        NotEnoughMoney,
        /// The owner of this asset doesn't intend to sell it
        AssetIsntForSale,
        /// While you waited, the asset was gone
        AssetAlreadySold,
        /// While you waited, the price has changed
        PriceChanged,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}


	}
}
