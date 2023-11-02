use candid::{CandidType, Nat, Principal};

pub type LenderId = u32;

#[allow(non_snake_case)]
#[derive(CandidType, Clone)]
pub struct Lender {
    pub id: LenderId,
    pub owner: Principal,
    pub from: Principal,
    pub fromAmount: Nat,
    pub to: Principal,
    pub toAmount: Nat,
}

#[derive(CandidType)]
pub struct Balance {
    pub owner: Principal,
    pub token: Principal,
    pub amount: Nat,
}

pub type CancelReceipt = Result<LenderId, CancelErr>;

#[derive(CandidType)]
pub enum CancelErr {
    NotAllowed,
    NotExistingOrder,
}
