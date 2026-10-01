/**
 * Example: Verify IPN webhook (use in your Express/Fastify handler)
 * Run: npx tsx examples/09-ipn-webhook.ts
 *
 * In real app:
 *   app.post('/webhook', async (req, res) => {
 *     const sig = req.headers['x-nowpayments-sig'] as string;
 *     if (!verifyIpnSignature(req.body, sig, process.env.IPN_SECRET!))
 *       return res.status(400).end();
 *     if (isPaymentComplete(req.body.payment_status)) { ... }
 *     res.status(200).end();
 *   });
 */
import {
  verifyIpnSignature,
  createIpnSignature,
  isPaymentComplete,
} from '../src';

const ipnSecret = process.env.IPN_SECRET || 'your_ipn_secret';

// Simulated IPN payload (real one comes from NOWPayments POST)
const mockPayload = {
  payment_id: '123',
  payment_status: 'finished',
  pay_address: '0x...',
  price_amount: 29.99,
  price_currency: 'usd',
  pay_amount: 0.001,
  actually_paid: 0.001,
  pay_currency: 'btc',
  order_id: 'order-1',
};

function main() {
  // For demo: create valid signature (in production NOWPayments sends it)
  const sig = createIpnSignature(mockPayload, ipnSecret);

  const ok = verifyIpnSignature(mockPayload, sig, ipnSecret);
  console.log('IPN signature valid?', ok);

  if (ok && isPaymentComplete(mockPayload.payment_status as 'finished')) {
    console.log('Handle: fulfill order', mockPayload.order_id);
  }
}

main();
