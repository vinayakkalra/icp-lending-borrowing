use candid::{CandidType, Decode, Encode};
use ic_storage::IcStorage;

#[derive(CandidType, Decode, Encode)]
struct Loan {
    id: u64,
    borrower: Principal,
    amount: u64,
    interest_rate: u64,
    term: u64,
    balance: u64,
    interest_accrued: u64,
}

pub struct BorrowingContract {
    storage: IcStorage,
    loans: Vec<Loan>,
}

impl BorrowingContract {
    pub fn new() -> BorrowingContract {
        BorrowingContract {
            storage: IcStorage::new(),
            loans: Vec::new(),
        }
    }

    pub fn request_loan(&mut self, borrower: Principal, amount: u64, interest_rate: u64, term: u64) -> Result<u64, String> {
        // Check if the user has sufficient collateral.
        let collateral_amount = self.get_collateral_amount(borrower);

        if collateral_amount < amount {
            return Err("Insufficient collateral".to_string());
        }

        // Transfer the borrowed tokens to the user's account.
        // ...

        // Calculate the interest rate for the loan.
        let interest_rate = utils::calculate_interest_rate(amount, interest_rate, term);

        // Create a new loan record.
        let new_loan = Loan {
            id: self.loans.len() as u64,
            borrower,
            amount,
            interest_rate,
            term,
            balance: amount,
            interest_accrued: 0,
        };

        // Add the new loan record to the list of loans.
        self.loans.push(new_loan);

        Ok(new_loan.id)
    }

    pub fn repay_loan(&mut self, loan_id: u64) -> Result<(), String> {
        // Get the loan record.
        let loan = self.get_loan(loan_id);

        // Check if the loan is fully repaid.
        if loan.balance == 0 {
            return Err("Loan already fully repaid".to_string());
        }

        // Repay the loan.
        // pending

        // Update the loan record.
        loan.balance = 0;
        loan.interest_accrued = 0;

        Ok(())
    }

    // ... Other functions ...to be decided
}
