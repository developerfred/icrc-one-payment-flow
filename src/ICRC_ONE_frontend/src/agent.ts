import { Actor, HttpAgent } from '@dfinity/agent';
import { idlFactory as payment_idl, canisterId as payment_canister_id } from 'dfx-generated/payment';

const agent = new HttpAgent();
const payment = Actor.createActor(payment_idl, { agent, canisterId: payment_canister_id });

export { payment };
