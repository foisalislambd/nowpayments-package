<?php
/**
 * Example: Check payment status
 * Run: php examples/02-check-status.php <payment_id>
 */

require __DIR__ . '/../vendor/autoload.php';

use NowPayments\NowPayments;
use NowPayments\Helpers;

$apiKey = getenv('NOWPAYMENTS_API_KEY') ?: 'your_api_key_here';
$paymentId = $argv[1] ?? null;

if (!$apiKey || $apiKey === 'your_api_key_here') {
    echo "Set NOWPAYMENTS_API_KEY environment variable.\n";
    exit(1);
}
if (!$paymentId) {
    echo "Usage: php examples/02-check-status.php <payment_id>\n";
    exit(1);
}

$np = new NowPayments([
    'apiKey' => $apiKey,
    'sandbox' => true,
]);

try {
    $payment = $np->getPaymentStatus($paymentId);
    echo Helpers::getPaymentSummary($payment) . "\n";
    echo "Status: " . Helpers::getStatusLabel($payment['payment_status']) . "\n";
    echo "Complete: " . (Helpers::isPaymentComplete($payment['payment_status']) ? 'yes' : 'no') . "\n";
} catch (\NowPayments\NowPaymentsError $e) {
    echo "Error: {$e->getMessage()}\n";
}
