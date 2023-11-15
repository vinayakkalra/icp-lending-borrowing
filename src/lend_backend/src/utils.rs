use candid::{CandidType, Decode, Encode};
use ic_storage::IcStorage;
use exchange_rates_canister::{ExchangeRatesCanister, get_exchange_rate};

pub struct UtilsContract {
    storage: IcStorage,
}

impl UtilsContract {
    pub fn new() -> UtilsContract {
        UtilsContract {
            storage: IcStorage::new(),
        }
    }

    pub fn calculate_interest_rate(&self, amount: u64, interest_rate: u64, term: u64) -> u64 {
        (amount * interest_rate * term) / 365
    }

    pub fn convert_tokens(&self, from_token: String, to_token: String, amount: u64) -> Result<u64, String> {
        let from_token_exchange_rate = get_exchange_rate(from_token.clone())?;
        let to_token_exchange_rate = get_exchange_rate(to_token.clone())?;

        let converted_amount = (amount * to_token_exchange_rate) / from_token_exchange_rate;

        Ok(converted_amount)
    }

    // ... Other functions ...to be decided
}
