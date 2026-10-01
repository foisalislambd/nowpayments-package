<?php
/**
 * Example: Create a payment
 * Run: php examples/01-create-payment.php
 */

require __DIR__ . '/../vendor/autoload.php';

use NowPayments\NowPayments;
use NowPayments\Helpers;

$apiKey = getenv('NOWPAYMENTS_API_KEY') ?: 'your_api_key_here';
if ($apiKey === 'your_api_key_here') {
    echo "Set NOWPAYMENTS_API_KEY environment variable.\n";
    exit(1);
}

$np = new NowPayments([
    'apiKey' => $apiKey,
    'sandbox' => true,
]);

try {
    $payment = $np->createPayment([
        'price_amount' => 29.99,
        'price_currency' => 'usd',
        'pay_currency' => 'btc',
        'order_id' => 'order-' . time(),
        'order_description' => 'Example payment',
    ]);

    echo "Payment created!\n";
    echo Helpers::getPaymentSummary($payment) . "\n";
    echo "Payment ID: {$payment['payment_id']}\n";
    echo "Pay: {$payment['pay_amount']} {$payment['pay_currency']} to {$payment['pay_address']}\n";
} catch (\NowPayments\NowPaymentsError $e) {
    echo "Error: {$e->getMessage()}\n";
    if ($e->getStatusCode()) {
        echo "Status: {$e->getStatusCode()}\n";
    }
}
