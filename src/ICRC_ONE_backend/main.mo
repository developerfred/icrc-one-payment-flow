import IC;

type Payment = {
  amount : Nat;
  payee : Principal;
};

public func process_payment(payment : Payment) : async Result<(), Text> {
  if (payment.amount > IC.ICPT.fromICPTs(100)) {
    return Err("Payment amount exceeds limit");
  }

  let caller = IC.caller();
  let balance = IC.canister_balance();

  if (payment.amount > balance) {
    return Err("Insufficient funds");
  }

  let res : Bool = IC
    .call
    .withPayment(payment.amount, payment.payee, encode())?;

  if (!res) {
    return Err("Payment failed");
  }

  Ok(())
}
