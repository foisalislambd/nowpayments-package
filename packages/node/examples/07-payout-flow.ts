/**
 * Example: Payout flow (validate address → create → verify)
 * Run: npx tsx examples/07-payout-flow.ts
 * Env: NOWPAYMENTS_API_KEY, EMAIL, PASSWORD, PAYOUT_ADDRESS, VERIFICATION_CODE?
 */
import { NowPayments } from '../src';

const np = new NowPayments({
  apiKey: process.env.NOWPAYMENTS_API_KEY!,
  sandbox: true,
});

const payCurrency = 'btc';
const payoutAddress = process.env.PAYOUT_ADDRESS || 'PASTE_BTC_ADDRESS';

async function main() {
  // 1. Validate payout address
  const valid = await np.validatePayoutAddress({
    address: payoutAddress,
    currency: payCurrency,
  });
  console.log('Address valid?', valid);

  // 2. Get JWT (payouts require auth)
  const email = process.env.EMAIL;
  const password = process.env.PASSWORD;
  if (!email || !password) {
    console.log('Set EMAIL and PASSWORD to create payout');
    return;
  }

  const { token } = await np.getAuthToken(email, password);

  // 3. Create payout
  const payout = await np.createPayout(
    {
      withdrawals: [
        { address: payoutAddress, currency: payCurrency, amount: 0.0001 },
      ],
      ipn_callback_url: 'https://yoursite.com/payout-webhook',
    },
    token
  );

  console.log('Payout created:', payout.id);
  console.log('Batch withdrawal ID:', payout.withdrawals?.[0]?.batch_withdrawal_id ?? payout.id);

  // 4. Verify (requires verification_code from email). Use payout.id as batch ID.
  const code = process.env.VERIFICATION_CODE;
  if (code) {
    const verified = await np.verifyPayout(payout.id, code, token);
    console.log('Verified:', verified);
  } else {
    console.log('Set VERIFICATION_CODE env to verify payout');
  }
}

main().catch(console.error);
