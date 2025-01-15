use candid::{CandidType, Deserialize, Principal};
use ic_cdk_macros::*;
use icrc_ledger_types::icrc1::{
    account::Account,
    transfer::{BlockIndex, NumTokens},  
};
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;

// Store balances for different accounts
thread_local! {
    static BALANCES: RefCell<HashMap<Account, NumTokens>> = RefCell::new(HashMap::new());
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct TransferArgs {
    amount: NumTokens,
    to_account: Account,
}

#[derive(CandidType, Deserialize)]
pub struct BalanceArgs {
    account: Account,
}

// Test function to add balance (now anyone can use it for testing)
#[update]
fn add_test_balance(account: Account, amount: NumTokens) -> Result<(), String> {
    BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();
        let balance = balances.entry(account).or_insert(NumTokens::from(0));
        *balance += amount;
    });

    Ok(())
}

#[update]
async fn transfer(args: TransferArgs) -> Result<BlockIndex, String> {
    let caller = get_caller();
    let from_account = Account {
        owner: caller,
        subaccount: None,
    };

    // Rest of the function remains the same
    let current_balance = get_balance_internal(&from_account);
    if current_balance < args.amount.clone() {
        return Err(format!(
            "Insufficient balance. Current balance: {}, Required: {}",
            current_balance, args.amount
        ));
    }

    BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();
        if let Some(balance) = balances.get_mut(&from_account) {
            *balance -= args.amount.clone();
        }
        let receiver_balance = balances.entry(args.to_account).or_insert(NumTokens::from(0));
        *receiver_balance += args.amount.clone();
    });

    Ok(BlockIndex::from(1))
}
// Function to check balance
#[query]
fn balance(args: BalanceArgs) -> NumTokens {
    get_balance_internal(&args.account)
}

// Helper function to get balance
fn get_balance_internal(account: &Account) -> NumTokens {
    BALANCES.with(|balances| {
        balances
            .borrow()
            .get(account)
            .cloned()
            .unwrap_or_else(|| NumTokens::from(0))
    })
}


#[cfg(test)]
mod mock_runtime {
    use candid::Principal;
    use std::cell::RefCell;
    
    thread_local! {
        static MOCK_USER: RefCell<Principal> = RefCell::new(Principal::anonymous());
    }

    pub fn set_caller(principal: Principal) {
        MOCK_USER.with(|user| {
            *user.borrow_mut() = principal;
        });
    }

    pub fn get_caller() -> Principal {
        MOCK_USER.with(|user| *user.borrow())
    }
}

// Then modify the main code to use a caller getter function
#[cfg(not(test))]
fn get_caller() -> Principal {
    ic_cdk::caller()
}

#[cfg(test)]
fn get_caller() -> Principal {
    mock_runtime::get_caller()
}



#[update]
fn deposit_notification(_from: Principal, amount: NumTokens) -> Result<(), String> {
    let to_account = Account {
        owner: get_caller(),
        subaccount: None,
    };

    BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();
        let balance = balances.entry(to_account).or_insert(NumTokens::from(0));
        *balance += amount;
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::mock_runtime::set_caller;

    // Helper function to create test principal
    fn test_principal(id: u8) -> Principal {
        Principal::from_slice(&[id; 29])
    }

    // Helper function to create test account
    fn test_account(id: u8) -> Account {
        Account {
            owner: test_principal(id),
            subaccount: None,
        }
    }

    #[test]
    fn test_add_test_balance() {
        let account = test_account(1);
        let amount = NumTokens::from(100);

        let result = add_test_balance(account.clone(), amount.clone());
        assert!(result.is_ok());

        let balance = get_balance_internal(&account);
        assert_eq!(balance, amount);
    }

    #[test]
    fn test_get_balance() {
        let account = test_account(2);
        let amount = NumTokens::from(150);

        let _ = add_test_balance(account.clone(), amount.clone());
        
        let balance_args = BalanceArgs {
            account: account.clone(),
        };
        
        let balance_result = balance(balance_args);
        assert_eq!(balance_result, amount);

        let non_existent = test_account(99);
        let zero_balance_args = BalanceArgs {
            account: non_existent,
        };
        let zero_balance_result = balance(zero_balance_args);
        assert_eq!(zero_balance_result, NumTokens::from(0));
    }

    #[test]
    fn test_deposit_notification() {
        let from = test_principal(3);
        let amount = NumTokens::from(200);
        let caller = test_principal(4);

        set_caller(caller);

        let result = deposit_notification(from, amount.clone());
        assert!(result.is_ok());

        let deposited_account = Account {
            owner: caller,
            subaccount: None,
        };
        
        let balance = get_balance_internal(&deposited_account);
        assert_eq!(balance, amount);
    }

    #[tokio::test]
    async fn test_transfer_success() {
        let from_principal = test_principal(5);
        let to_account = test_account(6);
        let initial_amount = NumTokens::from(500);
        let transfer_amount = NumTokens::from(200);

        let from_account = Account {
            owner: from_principal,
            subaccount: None,
        };
        let _ = add_test_balance(from_account.clone(), initial_amount.clone());

        set_caller(from_principal);

        let transfer_args = TransferArgs {
            amount: transfer_amount.clone(),
            to_account: to_account.clone(),
        };

        let result = transfer(transfer_args).await;
        assert!(result.is_ok());

        let from_balance = get_balance_internal(&from_account);
        let to_balance = get_balance_internal(&to_account);

        let expected_from_balance = initial_amount.clone() - transfer_amount.clone();
        assert_eq!(from_balance, expected_from_balance);
        assert_eq!(to_balance, transfer_amount);
    }

    #[tokio::test]
    async fn test_transfer_insufficient_balance() {
        let from_principal = test_principal(7);
        let to_account = test_account(8);
        let initial_amount = NumTokens::from(100);
        let transfer_amount = NumTokens::from(200);

        let from_account = Account {
            owner: from_principal,
            subaccount: None,
        };
        let _ = add_test_balance(from_account.clone(), initial_amount.clone());

        set_caller(from_principal);

        let transfer_args = TransferArgs {
            amount: transfer_amount,
            to_account,
        };

        let result = transfer(transfer_args).await;
        assert!(result.is_err());
        
        let from_balance = get_balance_internal(&from_account);
        assert_eq!(from_balance, initial_amount);
    }
}

// Export Candid interface
ic_cdk::export_candid!();