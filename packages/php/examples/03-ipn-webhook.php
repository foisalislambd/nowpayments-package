<?php
/**
 * Example: IPN webhook verification
 * Simulates verifying an IPN callback signature.
 * In production, use the raw request body and x-nowpayments-sig header.
 */

require __DIR__ . '/../vendor/autoload.php';

use NowPayments\Ipn;

$ipnSecret = getenv('NOWPAYMENTS_IPN_SECRET') ?: 'your_ipn_secret';

// Simulated payload (what NOWPayments sends to your webhook)
$payload = [
    'payment_id' => 123456,
    'payment_status' => 'finished',
    'pay_address' => 'bc1q...',
    'pay_amount' => 0.001,
    'pay_currency' => 'btc',
    'price_amount' => 29.99,
    'price_currency' => 'usd',
    'order_id' => 'order-123',
];

// Create a valid signature for testing
$signature = Ipn::createSignature($payload, $ipnSecret);

// Verify
$valid = Ipn::verifySignature($payload, $signature, $ipnSecret);
echo $valid ? "Signature valid!\n" : "Signature invalid!\n";

// Or with raw JSON string
$payloadStr = json_encode($payload);
$valid2 = Ipn::verifySignature($payloadStr, $signature, $ipnSecret);
echo $valid2 ? "String payload: valid!\n" : "String payload: invalid!\n";
