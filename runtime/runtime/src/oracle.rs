// use sr_primitives::traits::{Bounded, Dispatchable, OffchainWorker, One, Zero};
use support::{decl_event, decl_module, decl_storage};
use system::ensure_signed;

pub trait Trait: system::Trait {
    // TODO: Add other types and constants required configure this module.

    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as TemplateModule {
        // Just a dummy storage item.
        // Here we are declaring a StorageValue, `Something` as a Option<u32>
        // `get(something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
        Something get(something): Option<u32>;
    }
}

// The module's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing events
        // this is needed only if you are using events in your module
        fn deposit_event() = default;

        // Just a dummy entry point.
        // function that can be called by the external world as an extrinsics call
        // takes a parameter of the type `AccountId`, stores it and emits an event
        // pub fn do_something(origin, something: u32) -> Result {
        //     // TODO: You only need this if you want to check it was signed.
        //     let who = ensure_signed(origin)?;

        //     // TODO: Code to execute when something calls this.
        //     // For example: the following line stores the passed in u32 in the storage
        //     Something::put(something);

        //     // here we are raising the Something event
        //     Self::deposit_event(RawEvent::SomethingStored(something, who));
        //     Ok(())
        // }
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        // Just a dummy event.
        // Event `Something` is declared with a parameter of the type `u32` and `AccountId`
        // To emit this event, we call the deposit funtion, from our runtime funtions
        SomethingStored(u32, AccountId),
    }
);