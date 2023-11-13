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
pub struct Borrower {
    pub id: BorrowerId,
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


pub type DepositReceipt = Result<Nat, DepositErr>;

#[derive(CandidType)]
pub enum DepositErr {
    BalanceLow,
    TransferFailure,
}

pub type LendReceipt = Result<Option<Token>, LendErr>;

#[derive(CandidType)]
pub enum LendErr {
    InvalidLenderId,
    InvalidToken,
}

pub type BorrowReceipt = Result<Option<Token>, BorrowErr>;

#[derive(CandidType)]
pub enum BorrowErr {
    InvalidBorrowerId,
    InvalidToken,
}

pub type WithdrawReceipt = Result<Nat, WithdrawErr>;

#[derive(CandidType)]
pub enum WithdrawErr {
    BalanceLow,
    TransferFailure,
}