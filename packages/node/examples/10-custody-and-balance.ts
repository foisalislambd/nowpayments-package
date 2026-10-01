/**
 * Example: Balance + custody (sub-partner, deposit with payment, transfer)
 * Run: npx tsx examples/10-custody-and-balance.ts
 * Env: NOWPAYMENTS_API_KEY, EMAIL, PASSWORD (for createSubPartnerPayment)
 */
import { NowPayments } from '../src';

const np = new NowPayments({
  apiKey: process.env.NOWPAYMENTS_API_KEY!,
  sandbox: true,
});

async function main() {
  // Balance
  const balance = await np.getBalance();
  console.log('Balance:', balance);

  // Sub-partners (if using custody)
  const partners = await np.getSubPartners();
  console.log('Sub-partners:', partners);

  // Deposit with payment – top up sub-partner via crypto (requires JWT)
  const email = process.env.EMAIL;
  const password = process.env.PASSWORD;
  const subPartnerId = process.env.SUB_PARTNER_ID;

  if (email && password && subPartnerId) {
    const { token } = await np.getAuthToken(email, password);
    const { result } = await np.createSubPartnerPayment(
      { currency: 'trx', amount: 50, sub_partner_id: subPartnerId },
      token
    );
    console.log('Deposit payment:', result.pay_address, result.pay_amount, result.pay_currency);
  }
}

main().catch(console.error);
