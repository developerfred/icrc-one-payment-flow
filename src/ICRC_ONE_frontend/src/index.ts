import { ICRC_ONE_backend } from "../../declarations/ICRC_ONE_backend";

async function sendPayment(amount: string, payee: string): Promise<void> {
  const canisterId = '<COLOQUE AQUI O ID DO CANISTER>';

  const payment: Payment = {
    amount: BigInt(amount),
    payee: payee,
  };

  try {
    const result = await window.ic.plug.send(
      canisterId,
      'process_payment',
      payment
    );
    console.log(result);
  } catch (error) {
    console.error(error);
  }
}
