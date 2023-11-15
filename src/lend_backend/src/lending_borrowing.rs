// incorporating in a single canister


use candid::{CandidType, Decode, Encode};
use ic_storage::IcStorage;
use exchange_rates_canister::{ExchangeRatesCanister, get_exchange_rate};

#[derive(CandidType, Decode, Encode)]
struct Token {
    id: u64,
    name: String,
    symbol: String,
    total_supply: u64,
    owner: Principal,
}

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

#[derive(CandidType, Decode, Encode)]
struct Collateral {
    id: u64,
    owner: Principal,
    amount: u64,
}

pub struct LendingContract {
    storage: IcStorage,
    tokens: Vec<Token>,
    loans: Vec<Loan>,
    collaterals: Vec<Collateral>,
    exchange_rates_canister: ExchangeRatesCanister,
}

impl LendingContract {
    pub fn new(exchange_rates_canister: ExchangeRatesCanister) -> LendingContract {
        LendingContract {
            storage: IcStorage::new(),
            tokens: Vec::new(),
            loans: Vec::new(),
            collaterals: Vec::new(),
            exchange_rates_canister,
        }
    }

    // Token standard functions
    pub fn create_token(&mut self, name: String, symbol: String, total_supply: u64, owner: Principal) -> Result<u64, String> {
        // Check if the token name is already taken.
        for token in self.tokens.iter() {
            if token.name == name {
                return Err("Token name is already taken".to_string());
            }
        }

        // Create a new token record.
        let new_token = Token {
            id: self.tokens.len() as u64,
            name,
            symbol,
            total_supply,
            owner,
        };

        // Add the new token record to the list of tokens.
        self.tokens.push(new_token);

        Ok(new_token.id)
    }

    pub fn mint_tokens(&mut self, token_id: u64, owner: Principal, amount: u64) -> Result<(), String> {
        // Get the token record.
        let token = self.get_token(token_id);

        // Check if the owner of the token is authorized to mint new tokens.
        if token.owner != owner {
            return Err("Unauthorized to mint new tokens".to_string());
        }

        // Update the total supply of the token.
        token.total_supply += amount;

        Ok(())
    }

    pub fn burn_tokens(&mut self, token_id: u64, owner: Principal, amount: u64) -> Result<(), String> {
        // Get the token record.
        let token = self.get_token(token_id);

        // Check if the owner of the token has enough tokens to burn.
        if token.balance[owner] < amount {
            return Err("Insufficient balance to burn tokens".to_string());
        }

        // Update the balance of the token for the owner.
        token.balance[owner] -= amount;

        // Update the total supply of the token.
        token.total_supply -= amount;

        Ok(())
    }

    pub fn transfer_tokens(&mut self, token_id: u64, from: Principal, to: Principal, amount: u64) -> Result<(), String> {
        // Get the token record.
        let token = self.get_token(token_id);

        // Check if the sender of the tokens has enough tokens to transfer.
        if token.balance[from] < amount {
            return Err("Insufficient balance to transfer tokens".)
        }
    }
}

    // Collateral functions
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

        // Update the collateral record.
        let collateral_record = self.get_collateral_record(owner).unwrap();
        collateral_record.amount -= amount;

        Ok(amount)
    }

    pub fn release_collateral(&mut self, owner: Principal) -> Result<u64, String> {
        // Check if the user has sufficient collateral to withdraw.
        let collateral_amount = self.get_collateral_amount(owner);

        if collateral_amount == 0 {
            return Err("Insufficient collateral".to_string());
        }

        // Update the collateral record.
        let collateral_record = self.get_collateral_record(owner).unwrap();
        collateral_record.amount = 0;

        Ok(collateral_amount)
    }

    pub fn get_collateral_record(&self, owner: Principal) -> Option<&Collateral> {
        // Iterate over all collateral records and find the record for the owner.
        for collateral in self.collaterals.iter() {
            if collateral.owner == owner {
                return Some(collateral);
            }
        }

        None
    }

    pub fn get_collateral_amount(&self, owner: Principal) -> u64 {
        // Iterate over all collateral records and find the record for the owner.
        for collateral in self.collaterals.iter() {
            if collateral.owner == owner {
                return collateral.amount;
            }
        }

    }





// Lending functions

    // Lend tokens
    pub fn lend_tokens(&mut self, token_id: u64, lender: Principal, amount: u64) -> Result<(), String> {
        // Get the token record.
        let token = self.get_token(token_id);

        // Check if the lender has enough tokens to lend.
        if token.balance[lender] < amount {
            return Err("Insufficient balance to lend tokens".to_string());
        }

        // Update the balance of the token for the lender.
        token.balance[lender] -= amount;

        // Create a new collateral record.
        let new_collateral = Collateral {
            id: self.collaterals.len() as u64,
            owner: lender,
            amount,
        };

        // Add the new collateral record to the list of collaterals.
        self.collaterals.push(new_collateral);

        Ok(())
    }

    // Borrow tokens
    pub fn borrow_tokens(&mut self, borrower: Principal, token_id: u64, amount: u64) -> Result<u64, String> {
        // Calculate the LTV of the loan.
        let ltv = self.calculate_ltv(token_id, amount)?;

        // Check if the borrower has enough collateral to borrow the amount.
        let collateral_amount = self.get_collateral_amount(borrower);
        if collateral_amount < ltv {
            return Err("Insufficient collateral to borrow the amount".to_string());
        }

        // Create a new loan record.
        let new_loan = Loan {
            id: self.loans.len() as u64,
            borrower,
            amount,
            interest_rate: 10, // 10% interest rate
            term: 30, // 30 days term
            balance: amount,
            interest_accrued: 0,
            collateral_amount,
        };

        // Add the new loan record to the list of loans.
        self.loans.push(new_loan);

        Ok(new_loan.id)
    }

    // Repay a loan
    pub fn repay_loan(&mut self, loan_id: u64) -> Result<(), String> {
        // Get the loan record.
        let loan = self.get_loan(loan_id);

        // Check if the loan is paid off.
        if loan.balance == 0 {
            return Err("Loan is already paid off".to_string());
        }

        // Repay the loan.
        self.burn_tokens(loan.token_id, loan.borrower, loan.balance);
        self.release_collateral(loan.borrower);
        self.loans.remove(loan_id as usize);

        Ok(())
    }

    // Liquidate a borrow position
    pub fn liquidate_borrow_position(&mut self, loan_id: u64) -> Result<(), String> {
        // Get the loan record.
        let loan = self.get_loan(loan_id);

        // Check if the loan is in default.
        if loan.balance > loan.collateral_amount {
            return Err("Loan is in default".to_string());
        }

        // Liquidate the collateral to repay the loan.
        self.burn_tokens(loan.token_id, loan.borrower, loan.collateral_amount);
        self.loans.remove(loan_id as usize);

        Ok(())
    }

    // Handle margin calls
    pub fn handle_margin_calls(&mut self) {
        // Iterate over all loans and check for margin calls.
        for loan in self.loans.iter() {
            if loan.balance > loan.collateral_amount * 0.8 {
                // Liquidate the loan.
                self.liquidate_borrow_position(loan.id).unwrap();
            }
        }
    }

