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

	use frame_support::{
        pallet_prelude::*,
        traits::{tokens::ExistenceRequirement, Currency},
    };
	use frame_support::sp_runtime::ArithmeticError;
	use frame_system::ensure_signed;
	use frame_system::pallet_prelude::OriginFor;


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


	/// Keeps track of the number of assets in existence.
    #[pallet::storage]
    pub(super) type CountForAssets<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// Maps the asset struct to the asset id.
    #[pallet::storage]
    pub(super) type Assets<T: Config> = StorageMap<_, Twox64Concat, u32, Asset<T>>;

    /// Track the assets owned by each account.
    #[pallet::storage]
    pub(super) type AssetsOwned<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, BoundedVec<u32, T::MaxAssetsOwned>, ValueQuery>;


	// Our pallet's genesis configuration
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub assets: Vec<(T::AccountId, u32)>,
    }

    // Required to implement default for GenesisConfig
    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> GenesisConfig<T> {
            GenesisConfig {
                assets: vec![],
            }
        }
    }



	#[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            for (account, id) in &self.assets {
                assert!(Pallet::<T>::mintasset(account, *id,).is_ok());
            }
        }
    }



	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
        /// A new asset was successfully created.
        AssetCreated { asset_id: u32, owner: T::AccountId },
        /// The price of an asset was successfully set.
        AssetPriceSet {
            asset_id: u32,
            price: Option<BalanceOf<T>>,
        },
        /// An asset was successfully transferred.
        AssetTransferred {
            from: T::AccountId,
            to: T::AccountId,
            asset_id: u32,
        },
        /// An asset was successfully sold.
        AssetSold {
            seller: T::AccountId,
            buyer: T::AccountId,
            asset_id: u32,
            price: BalanceOf<T>,
        },

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


	#[pallet::call]
    impl<T: Config> Pallet<T> {



        /// Create a new asset.
        ///
        /// The actual asset creation is done in the `mintasset()` function.
        #[pallet::weight(0)]
        pub fn create_asset(origin: OriginFor<T>, asset_id: u32) -> DispatchResult {
            // Make sure the caller is from a signed origin
            let sender = ensure_signed(origin)?;

            // Write new asset to storage by calling helper function
            Self::mintasset(&sender, asset_id)?;

            Ok(())
        }





        /// Directly transfer an asset to another recipient.
        ///
        /// Any account that holds an asset can send it to another account. This will reset the
        /// asking price of the asset, marking it not for sale.
        #[pallet::weight(0)]
        pub fn asset_transfer(
            origin: OriginFor<T>,
            to: T::AccountId,
            asset_id: u32,
        ) -> DispatchResult {
            // Make sure the caller is from a signed origin
            let from = ensure_signed(origin)?;
            let asset = Assets::<T>::get(&asset_id).ok_or(Error::<T>::NoAsset)?;
            ensure!(asset.owner == from, Error::<T>::NotOwner);
            Self::do_transfer_asset(asset_id, to, None)?;
            Ok(())
        }



        /// Buy a saleable asset. The bid price provided from the buyer has to be equal or higher
        /// than the ask price from the seller.
        ///
        /// This will reset the asking price of the asset, marking it not for sale.
        /// Marking this method `transactional` so when an error is returned, we ensure no storage
        /// is changed.
        #[pallet::weight(0)]
        pub fn buy_asset(
            origin: OriginFor<T>,
            asset_id: u32,
            bid_price: BalanceOf<T>,
        ) -> DispatchResult {
            // Make sure the caller is from a signed origin
            let buyer = ensure_signed(origin)?;
            // Transfer the asset from seller to buyer as a sale.
            Self::do_transfer_asset(asset_id, buyer, Some(bid_price))?;

            Ok(())
        }





        /// Set the price for an asset.
        ///
        /// Updates asset price and updates storage.
        #[pallet::weight(0)]
        pub fn set_asset_price(
            origin: OriginFor<T>,
            asset_id: u32,
            new_price: Option<BalanceOf<T>>,
        ) -> DispatchResult {
            // Make sure the caller is from a signed origin
            let sender = ensure_signed(origin)?;

            // Ensure the asset exists and is called by the asset owner
            let mut asset = Assets::<T>::get(&asset_id).ok_or(Error::<T>::NoAsset)?;
            ensure!(asset.owner == sender, Error::<T>::NotOwner);

            // Set the price in storage
            asset.price = new_price;
            Assets::<T>::insert(&asset_id, asset);

            // Deposit a "PriceSet" event.
            Self::deposit_event(Event::AssetPriceSet {
                asset_id: asset_id,
                price: new_price,
            });

            Ok(())
        }
    }


	  //** Our helper functions.**//

    impl<T: Config> Pallet<T> {

        // Helper to mint an asset
        pub fn mintasset(owner: &T::AccountId, asset_id: u32) -> Result<u32, DispatchError> {
            // Create a new asset
            let asset = Asset::<T> {
                asset_id,
                price: None,
                owner: owner.clone(),
            };

            // Check if the asset does not already exist in our storage map
            ensure!(
                !Assets::<T>::contains_key(&asset.asset_id),
                Error::<T>::DuplicateAsset
            );

            // Performs this operation first as it may fail
            let count = CountForAssets::<T>::get();
            let new_count = count.checked_add(1).ok_or(ArithmeticError::Overflow)?;

            // Append asset to AssetsOwned
            AssetsOwned::<T>::try_append(&owner, asset.asset_id)
                .map_err(|_| Error::<T>::TooManyOwned)?;

            // Write new asset to storage
            Assets::<T>::insert(asset.asset_id, asset);
            CountForAssets::<T>::put(new_count);

            // Deposit our "Created" event.
            Self::deposit_event(Event::AssetCreated {
                asset_id: asset_id,
                owner: owner.clone(),
            });

            // Returns the id of the new asset if this succeeds
            Ok(asset_id)
        }

        // Update storage to transfer asset
        pub fn do_transfer_asset(
            asset_id: u32,
            to: T::AccountId,
            maybe_bid_price: Option<BalanceOf<T>>,
        ) -> DispatchResult {
            // Get the asset
            let mut asset = Assets::<T>::get(&asset_id).ok_or(Error::<T>::NoAsset)?;
            let from = asset.owner;

            ensure!(from != to, Error::<T>::TransferToSelf);
            let mut from_owned = AssetsOwned::<T>::get(&from);

            // Remove asset from the list of owned assets.
            if let Some(ind) = from_owned.iter().position(|&id| id == asset_id) {
                from_owned.swap_remove(ind);
            } else {
                return Err(Error::<T>::NoAsset.into());
            }

            // Add asset to the list of owned assets.
            let mut to_owned = AssetsOwned::<T>::get(&to);
            to_owned
                .try_push(asset_id)
                .map_err(|()| Error::<T>::TooManyOwned)?;

            // Mutating state here via a balance transfer, so nothing is allowed to fail after this.
            if let Some(bid_price) = maybe_bid_price {
                if let Some(price) = asset.price {
                    ensure!(bid_price >= price, Error::<T>::BidPriceTooLow);
                    // Transfer the amount from buyer to seller
                    T::Currency::transfer(&to, &from, price, ExistenceRequirement::KeepAlive)?;
                    // Deposit sold event
                    Self::deposit_event(Event::AssetSold {
                        seller: from.clone(),
                        buyer: to.clone(),
                        asset_id: asset_id,
                        price,
                    });
                } else {
                    return Err(Error::<T>::NotForSale.into());
                }
            }

            // Transfer succeeded, update the asset owner and reset the price to `None`.
            asset.owner = to.clone();
            asset.price = None;

            // Write updates to storage
            Assets::<T>::insert(&asset_id, asset);
            AssetsOwned::<T>::insert(&to, to_owned);
            AssetsOwned::<T>::insert(&from, from_owned);

            Self::deposit_event(Event::AssetTransferred {
                from,
                to,
                asset_id: asset_id,
            });

            Ok(())
        }

    }

}
