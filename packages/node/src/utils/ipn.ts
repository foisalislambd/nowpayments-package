/**
 * IPN (Instant Payment Notification) verification utilities
 * Matches official docs: sort keys recursively, then HMAC-SHA512
 * @see https://nowpayments.io/help/payments/api
 */

import * as crypto from 'crypto';

/** Recursively sort object keys (matches NOWPayments IPN Node example).
 * Arrays are treated as objects (official algorithm uses typeof === 'object').
 */
function sortObject(obj: Record<string, unknown>): Record<string, unknown> {
  return Object.keys(obj)
    .sort()
    .reduce(
      (result, key) => {
        const val = obj[key];
        result[key] =
          val != null && typeof val === 'object'
            ? (sortObject(val as Record<string, unknown>) as unknown)
            : val;
        return result;
      },
      {} as Record<string, unknown>
    );
}

/**
 * Verify IPN callback signature from NOWPayments.
 * Safe to call – handles invalid input gracefully.
 *
 * Prefer the raw HTTP body string (before JSON parsing) when possible.
 * Parsed objects from Express/Fastify can change number/string types and break verification.
 *
 * @param payload - Raw request body string (recommended) or parsed object
 * @param signature - Value from x-nowpayments-sig header
 * @param ipnSecret - Your IPN Secret from Dashboard → Store Settings
 * @returns true if signature is valid, false otherwise
 */
export function verifyIpnSignature(
  payload: string | Record<string, unknown>,
  signature: string,
  ipnSecret: string
): boolean {
  if (!signature?.trim() || !ipnSecret?.trim()) {
    return false;
  }

  try {
    let obj: Record<string, unknown>;
    if (typeof payload === 'string') {
      obj = JSON.parse(payload) as Record<string, unknown>;
    } else if (payload && typeof payload === 'object') {
      obj = payload;
    } else {
      return false;
    }
    const jsonString = JSON.stringify(sortObject(obj));

    const hmac = crypto.createHmac('sha512', ipnSecret.trim());
    hmac.update(jsonString);
    const computedSig = hmac.digest('hex');

    const sigRaw = signature.trim().toLowerCase();
    const sigHex = (sigRaw.includes('=') ? sigRaw.split('=').pop()! : sigRaw).replace(
      /[^a-f0-9]/g,
      ''
    );
    const computedHex = computedSig.toLowerCase();
    const sigBuf = Buffer.from(sigHex, 'hex');
    const computedBuf = Buffer.from(computedHex, 'hex');
    if (sigBuf.length !== computedBuf.length || sigBuf.length === 0) return false;

    return crypto.timingSafeEqual(sigBuf, computedBuf);
  } catch {
    return false;
  }
}

/**
 * Create IPN signature for testing (e.g., mocking callbacks)
 */
export function createIpnSignature(
  payload: Record<string, unknown>,
  ipnSecret: string
): string {
  const jsonString = JSON.stringify(sortObject(payload));
  const hmac = crypto.createHmac('sha512', ipnSecret.trim());
  hmac.update(jsonString);
  return hmac.digest('hex');
}
