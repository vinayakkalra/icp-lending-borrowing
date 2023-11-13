use candid::{CandidType, Decode, Encode};
use ic_storage::IcStorage;

#[derive(CandidType, Decode, Encode)]
struct Collateral {
    id: u64,
    owner: Principal,
    amount: u64,
}

pub struct CollateralContract {
    storage: IcStorage,
    collaterals: Vec<Collateral>,
}

impl CollateralContract {
    pub fn new() -> CollateralContract {
        CollateralContract {
            storage: IcStorage::new(),
            collaterals: Vec::new(),
        }
    }

    pub fn deposit_collateral(&mut self, owner: Principal, amount: u64) -> Result<u64, String> {
        // Check if the user already has a collateral record.
        let collateral_record = self.get_collateral_record(owner);

        if collateral_record.is_some() {
            // Update the existing collateral record.
            collateral_record.unwrap().amount += amount;
        } else {
            // Create a new collateral record.
            let new_collateral = Collateral {
                id: self.collaterals.len() as u64,
                owner,
                amount,
            };

            // Add the new collateral record to the list of collaterals.
            self.collaterals.push(new_collateral);
        }

        Ok(amount)
    }

    pub fn withdraw_collateral(&mut self, owner: Principal, amount: u64) -> Result<u64, String> {
        // Check if the user has sufficient collateral to withdraw.
        let collateral_amount = self.get_collateral_amount(owner);

        if collateral_amount < amount {
            return Err("Insufficient collateral".to_string());
        }
    }}
        
