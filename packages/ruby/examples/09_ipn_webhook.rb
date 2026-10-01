# frozen_string_literal: true

# IPN webhook verification example
# In production, use in your Sinatra/Rails webhook endpoint

require_relative "../lib/nowpayments"

ipn_secret = ENV["NOWPAYMENTS_IPN_SECRET"] || "your_ipn_secret"
np = NowPayments::Client.new(api_key: "dummy", ipn_secret: ipn_secret)

# Simulated webhook payload (from NOWPayments)
payload = {
  "payment_id" => 5524759814,
  "payment_status" => "finished",
  "pay_address" => "bc1q...",
  "pay_amount" => 0.001,
  "pay_currency" => "btc"
}

# Create valid signature for demo
signature = NowPayments::IPN.create_signature(payload, ipn_secret)
puts "Generated signature: #{signature}"

# Verify
valid = np.verify_ipn(payload, signature)
puts "Verification: #{valid ? 'VALID' : 'INVALID'}"

# Or use standalone
valid2 = NowPayments::IPN.verify_signature(payload, signature, ipn_secret)
puts "Standalone verification: #{valid2 ? 'VALID' : 'INVALID'}"
