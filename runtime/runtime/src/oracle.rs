use app_crypto::{KeyTypeId, RuntimeAppPublic};
use codec::{Decode, Encode};
use primitives::offchain::{Duration, HttpRequestId, HttpRequestStatus};
use rstd::result::Result;
use rstd::vec::Vec;
// use serde_json::serde::{Deserialize, Serialize};
use sr_primitives::traits::Member;
use support::{decl_event, decl_module, decl_storage, ensure, Parameter, StorageMap, StorageValue};
use system::offchain::SubmitUnsignedTransaction;
use system::{ensure_none, ensure_signed};
/// only for debug
fn debug(msg: &str) {
    // let msg = format!("\x1b[34m{}", msg);
    runtime_io::print_utf8(msg.as_bytes());
}

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
    // pub type AuthoritySignature = app_sr25519::Signature;

    /// An oracle identifier using sr25519 as its crypto.
    pub type AuthorityId = app_sr25519::Public;
}

// TODO: add Value to Trait, config outside
pub type Value = u32;

// TODO: BTCValue is just an example, feel free to replace it with another name
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub struct BTCValue<BlockNumber>
where
    BlockNumber: PartialEq + Eq + Decode + Encode,
{
    block_number: BlockNumber,
    price: Value,
}

#[derive(Encode, Decode, Clone, Debug)]
pub enum PriceFrom {
    CoinMarketCap,
    CoinDesk,
    Binance,
}

pub trait Trait: timestamp::Trait {
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

        /// Provide price value for external api consuming
        pub PriceValue get(price_value): Option<Value>;

        /// Values for specific block_number
        pub Values get(values): map T::BlockNumber => Option<BTCValue<T::BlockNumber>>;

    }
}

decl_event!(
    pub enum Event<T>
    where
        <T as system::Trait>::AccountId,
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

        // Runs after every block.
        fn offchain_worker(now: T::BlockNumber) {
            // FIXME: only request a series of request at once
            let block_number = Self::block_number();
            if let Some(block_number) = block_number {
                let value = Self::values(block_number);
                if value.is_some() {
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

        fn verify_value(origin, value: BTCValue<T::BlockNumber>
            // signature: <T::AuthorityId as RuntimeAppPublic>::Signature
        ) {
            ensure_none(origin)?;

            // verify the signature
            let _public = Self::authorised_key();
            // TODO: public doesn't have `verify` function
            // let signature_valid = value.using_encoded(|encoded_value| {
            //     public.verify(&encoded_value, &signature)
            // });
            // ensure!(signature_valid, "Invalid value signature.");

            // update value in storage
            <Values<T>>::insert(value.block_number, &value);
            <PriceValue>::put(value.price);

            Self::deposit_event(RawEvent::UpdateValue(value.price));
        }
    }
}

impl<T: Trait> Module<T> {
    fn offchain(now: T::BlockNumber) {
        <BlockNumber<T>>::put(now);

        Self::request_cmc_value();
        Self::request_cds_value();
    }

    fn request_cmc_value() {
        // TODO: use offchain http request to get btc/usdt price
        // TODO: uri and api key should write into sotrage like authorisedKey
        let uri = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest?id=1";
        let api_key_value = "20a084fd-afdd-4c81-8e95-08868a45fcaf";
        let api_key = "X-CMC_PRO_API_KEY";

        let res = Self::http_request_get(uri, Some((api_key, api_key_value)));
        match res {
            Ok(_buf) => (), // TODO: how to parse the result `data[1].quote.USD.price`
            Err(_) => debug("parse body failed"),
        }

        // FIXME: Get price from response
        let price = 8600;
        Self::update_value(price);
    }

    fn request_csd_value() {
        let uri = "https://api.coindesk.com/v1/bpi/currentprice/USD.json";
        let res = Self::http_request_get(uri, None);
        match res {
            Ok(_buf) => (),
            Err(_) => debug("parse body failed"),
        }

        // FIXME: Get price from response
        let price = 8600;
        Self::update_value(price);
    }

    fn http_request_get(
        uri: &str,
        header: Option<(&str, &str)>,
    ) -> Result<[u8; 2048], &'static str> {
        // TODO: extract id, maybe use for other place
        let id: HttpRequestId = runtime_io::http_request_start("GET", uri, &[0]).unwrap();
        let deadline = runtime_io::timestamp().add(Duration::from_millis(10_000));

        if let Some((name, value)) = header {
            match runtime_io::http_request_add_header(id, name, value) {
                Err(_) => debug("Add request header failed"),
                Ok(_) => debug("Add request header succeed"),
            };
        }

        match runtime_io::http_response_wait(&[id], Some(deadline))[0] {
            HttpRequestStatus::Finished(200) => (),
            _ => return Err("Request failed"),
        }

        // set a fix len for result
        let buffer_len = 2048;
        let mut buf = Vec::with_capacity(buffer_len as usize);
        buf.resize(buffer_len as usize, 0);

        let res = runtime_io::http_response_read_body(id, &mut buf, Some(deadline));
        match res {
            Ok(_len) => {
                let result = &buf[..buffer_len];
                runtime_io::print_utf8(result);

                let mut res: [u8; 2048] = [0; 2048];
                res.copy_from_slice(result);
                return Ok(res);
            }
            Err(_) => return Err("parse body failed"),
        }
    }

    fn update_value(value: Value) -> Result<(), &'static str> {
        let block_number = Self::block_number();
        ensure!(block_number.is_some(), "block number can not be empty");
        let block_number = block_number.unwrap();

        let key = Self::authorised_key();
        if let Some(_key) = key {
            let btc_value = BTCValue {
                block_number,
                price: value,
            };
            // TODO: key doesn't have `sign` function
            // let signature = key.sign(&value.encode()).ok_or("Offchain error: signing failed!")?;
            let call = Call::verify_value(btc_value);

            // submit unsigned transaction
            let result = T::SubmitTransaction::submit_unsigned(call);
            match result {
                Ok(_) => runtime_io::print_utf8(b"execute off-chain worker success"),
                Err(_) => runtime_io::print_utf8(b"execute off-chain worker failed!"),
            }
        } else {
            runtime_io::print_utf8(b"No authorised key found!");
        }

        Ok(())
    }
}
