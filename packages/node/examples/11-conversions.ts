/**
 * Example: Create conversion (custody) and check status
 * Run: npx tsx examples/11-conversions.ts
 * Env: NOWPAYMENTS_API_KEY, EMAIL, PASSWORD (JWT required)
 */
import { NowPayments } from '../src';

const np = new NowPayments({
  apiKey: process.env.NOWPAYMENTS_API_KEY!,
  sandbox: true,
});

async function main() {
  const email = process.env.EMAIL;
  const password = process.env.PASSWORD;
  if (!email || !password) {
    console.log('Set EMAIL and PASSWORD (conversions require JWT)');
    return;
  }

  const { token } = await np.getAuthToken(email, password);

  const conv = await np.createConversion(
    {
      amount: 0.001,
      from_currency: 'btc',
      to_currency: 'usd',
    },
    token
  ) as { deposit_id?: string; pay_address?: string; pay_amount?: number; pay_currency?: string; to_amount?: number; to_currency?: string };

  console.log('Conversion:', conv);
  if (conv?.deposit_id) {
    const status = await np.getConversionStatus(conv.deposit_id, token);
    console.log('Status:', status);
  }
}

main().catch(console.error);
