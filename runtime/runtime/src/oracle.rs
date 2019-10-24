use codec::{Encode, Decode};
use sr_primitives::traits::{Member};
use support::{decl_event, decl_module, decl_storage, Parameter, ensure};
use system::offchain::{SubmitUnsignedTransaction};
use app_crypto::{KeyTypeId, RuntimeAppPublic};
use system::{ensure_none, ensure_signed};

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"orin");

pub mod sr25519 {
	mod app_sr25519 {
		use app_crypto::{app_crypto, sr25519};
		app_crypto!(sr25519, super::super::KEY_TYPE);

		impl From<Signature> for sr_primitives::AnySignature {
			fn from(sig: Signature) -> Self {
				sr25519::Signature::from(sig).into()
			}
		}
	}

	/// An oracle signature using sr25519 as its crypto.
	pub type AuthoritySignature = app_sr25519::Signature;

	/// An oracle identifier using sr25519 as its crypto.
	pub type AuthorityId = app_sr25519::Public;
}

// TODO: add Value to Trait, config outside
pub type Value = u32;

// TODO: BTCValue is just an example, feel free to replace it with another name 
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
pub struct BTCValue<BlockNumber>
	where BlockNumber: PartialEq + Eq + Decode + Encode,
{
	block_number: BlockNumber,
	value: Value,
}


pub trait Trait: system::Trait {
    /// The identifier type for an authority.
	type AuthorityId: Member + Parameter + RuntimeAppPublic + Default + Ord;
    	
    /// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	/// A dispatchable call type.
	type Call: From<Call<Self>>;
	
    /// A transaction submitter.
	type SubmitTransaction: SubmitUnsignedTransaction<Self, <Self as Trait>::Call>;
}

// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as Oracle {
        /// The key used to sign the payload
        /// TODO: the type may change to `AuthorityId`
        pub AuthorisedKey get(authorised_key): Option<T::AccountId>;

        pub BlockNumber get(block_number): Option<T::BlockNumber>;

        /// values for specific block_number
        pub Values get(values): map T::BlockNumber => Option<BTCValue<T::BlockNumber>>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        SetAuthority(AccountId),
        UpdateValue(Value),
    }
);

// The module's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing events
        fn deposit_event() = default;

        verify_value(
            origin,
			value: BTCValue<T::BlockNumber>,
			signature: <T::AuthorityId as RuntimeAppPublic>::Signature) 
        {
            ensure_none(origin)?;

            // verify the signature
            let public = Self::authorised_key();
            let signature_valid = value.using_encoded(|encoded_value| {
			    public.verify(&encoded_value, &signature)
			});
			ensure!(signature_valid, "Invalid value signature.");

            // update value in storage
            <Values<T>>::insert(value.block_number, value);

            Self::deposit_event(RawEvent::UpdateValue(10));
        }

        // Runs after every block.
		fn offchain_worker(now: T::BlockNumber) {
            let block_number = Self::block_number();
            if let Some(block_number) = block_number {
                let value = Self::values(block_number);
                if Some(value) = value {
                    Self::offchain(now);
                }
            } else {
                Self::offchain(now);
            }
		}

		// Simple authority management: init authority key
		pub fn set_authority(origin) {
			// Should be protected by a root-call (e.g. through governance like `sudo`).
			// TODO: let sender = ensure_root(origin)?;
			let sender = ensure_signed(origin)?;

			<AuthorisedKey<T>>::put(sender.clone());

			Self::deposit_event(RawEvent::SetAuthority(sender));
		}
    }
}

impl<T: Trait> Module<T> {
	fn offchain(now: T::BlockNumber) {
        <BlockNumber<T>>::put(now);
        Self::request_value(now);
	}

    fn request_value(now: T::BlockNumber) {
        <Values<T>>::insert(now, None);

        // TODO: use offchain http request to get btc/usdt price
        let value = 10;
        Self::update_value(value)
    }

    fn update_value(value: Value) {
        let block_number = Self::block_number();
        ensure!(block_number.is_some(), "block number can not be empty");

        let key = Self::authorised_key();
		if let Some(key) = key {
            let value = BTCValue { block_number, value};
            let signature = key.sign(&value.encode()).ok_or("Offchain error: signing failed!")?;
			let call = T::Call::verify_value(value, signature);

			// submit unsigned transaction
			let result = T::SubmitTransaction::submit_unsigned(call);
			match result {
				Ok(_a) => runtime_io::print_utf8(b"execute off-chain worker success"),
				Err(_b) => runtime_io::print_utf8(b"execute off-chain worker failed!"),
			}
			
		} else {
			runtime_io::print_utf8(b"No authorised key!");
		}
    }
}