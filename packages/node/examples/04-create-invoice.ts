/**
 * Example: Create invoice (redirect customer to URL)
 * Run: npx tsx examples/04-create-invoice.ts
 */
import { NowPayments } from '../src';

const np = new NowPayments({
  apiKey: process.env.NOWPAYMENTS_API_KEY!,
  sandbox: true,
});

async function main() {
  const invoice = await np.createInvoice({
    price_amount: 49.99,
    price_currency: 'usd',
    pay_currency: 'btc',
    order_id: 'inv-' + Date.now(),
    order_description: 'Premium subscription',
    success_url: 'https://yoursite.com/success',
    cancel_url: 'https://yoursite.com/cancel',
  });

  console.log('Invoice created!');
  console.log('Redirect customer to:', invoice.invoice_url);
}

main().catch(console.error);
