use ic_cdk_macros::*;
use ic_cdk::api::{caller, token::ICPT};
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_cdk::export::Principal;

#[derive(CandidType, Deserialize)]
struct Payment {
    amount: ICPT,
    payee: Principal,
}

#[update]
fn process_payment(payment: Payment) -> Result<(), String> {
    if payment.amount > ICPT::from_icpts(100) {
        return Err(String::from("Payment amount exceeds limit"));
    }

    let caller = caller();
    let balance = ICPT::from_cents(ic_cdk::api::canister_balance());

    if payment.amount > balance {
        return Err(String::from("Insufficient funds"));
    }

    ic_cdk::api::call::call_with_payment::<()>(&payment.payee, payment.amount, ()).map_err(|e| format!("{:?}", e))?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use ic_cdk::{api::canister_balance, export::Principal};

    #[test]
    fn test_process_payment_sufficient_funds() {
        let amount = ICPT::from_icpts(50);
        let payee = Principal::from_text("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=").unwrap();

        let payment = Payment { amount, payee };

        process_payment(payment).unwrap();
    }

    #[test]
    fn test_process_payment_insufficient_funds() {
        let amount = ICPT::from_icpts(500);
        let payee = Principal::from_text("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=").unwrap();

        let payment = Payment { amount, payee };

        let result = process_payment(payment);
        assert_eq!(result, Err(String::from("Insufficient funds")));
    }

    #[test]
    fn test_process_payment_exceeds_limit() {
        let amount = ICPT::from_icpts(150);
        let payee = Principal::from_text("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=").unwrap();

        let payment = Payment { amount, payee };

        let result = process_payment(payment);
        assert_eq!(result, Err(String::from("Payment amount exceeds limit")));
    }
}
