use frame_support::traits::UnixTime;

#[pallet::config]
pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    type TimeProvider: UnixTime;
}

let time: u64 = T::TimeProvider::now();

impl my_pallet::Config for Runtime {
    type Event = Event;
    type TimeProvider = pallet_timestamp::Pallet<Runtime>;
    // Or more easily just `Timestamp` assuming you used that name in `construct_runtime!`
}

impl<T: Config> Time for Pallet<T> {
    type Moment = T::Moment;

    /// Before the first set of now with inherent the value returned is zero.
    fn now() -> Self::Moment {
        Self::now()
    }
}

/// Before the timestamp inherent is applied, it returns the time of previous block.
///
/// On genesis the time returned is not valid.
impl<T: Config> UnixTime for Pallet<T> {
    fn now() -> core::time::Duration {
        // now is duration since unix epoch in millisecond as documented in
        // `sp_timestamp::InherentDataProvider`.
        let now = Self::now();
        sp_std::if_std! {
            if now == T::Moment::zero() {
                log::error!(
                    target: "runtime::timestamp",
                    "`pallet_timestamp::UnixTime::now` is called at genesis, invalid value returned: 0",
                );
            }
        }
        core::time::Duration::from_millis(now.saturated_into::<u64>())
    }
}