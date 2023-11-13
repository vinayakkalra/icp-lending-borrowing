use candid::{CandidType, Decode, Encode};
use ic_storage::IcStorage;

#[derive(CandidType, Decode, Encode)]
struct Loan {
    id: u64,
    borrower: Principal,
    lender: Principal,
    amount: u64,
    interest_rate: u64,
    term: u64,
    balance: u64,
}

pub struct LendingContract {
    storage: IcStorage,
    loans: Vec<Loan>,
}

impl LendingContract {
    pub fn new() -> LendingContract {
        LendingContract {
            storage: IcStorage::new(),
            loans: Vec::new(),
        }
    }

    pub fn create_loan(&mut self, borrower: Principal, lender: Principal, amount: u64, interest_rate: u64, term: u64) -> Result<u64, String> {
        let new_loan = Loan {
            id: self.loans.len() as u64,
            borrower,
            lender,
            amount,
            interest_rate,
            term,
            balance: amount,
        };

        self.loans.push(new_loan);

        Ok(new_loan.id)
    }

    pub fn get_loan(&self, id: u64) -> Result<Loan, String> {
        self.loans.get(id).ok_or("Loan not found".to_string())
    }

    pub fn deposit_assets(&mut self, loan_id: u64, amount: u64) -> Result<(), String> {
        let mut loan = self.get_loan(loan_id)?;

        loan.balance += amount;

        self.loans[loan_id] = loan;

        Ok(())
    }

    pub fn withdraw_assets(&mut self, loan_id: u64, amount: u64) -> Result<(), String> {
        let mut loan = self.get_loan(loan_id)?;

        if loan.balance < amount {
            return Err("Insufficient funds".to_string());
        }

        loan.balance -= amount;

        self.loans[loan_id] = loan;

        Ok(())
    }

    pub fn repay_loan(&mut self, loan_id: u64) -> Result<(), String> {
        let mut loan = self.get_loan(loan_id)?;

        if loan.balance > 0 {
            return Err("Loan not fully repaid".to_string());
        }

        self.loans.remove(loan_id);

        Ok(())
    }
}

pub fn calculate_interest_rate(principal: u64, interest_rate: u64, time_in_days: u64) -> u64 {
    (principal * interest_rate * time_in_days) / 365
}
