# frozen_string_literal: true

# Payout flow (validate address → create → verify)
# Run: ruby examples/07_payout_flow.rb
# Env: NOWPAYMENTS_API_KEY, EMAIL, PASSWORD, PAYOUT_ADDRESS, VERIFICATION_CODE?

require_relative "../lib/nowpayments"

api_key = ENV["NOWPAYMENTS_API_KEY"] || "your_api_key"
np = NowPayments::Client.new(api_key: api_key, sandbox: true)

pay_currency = "btc"
payout_address = ENV["PAYOUT_ADDRESS"] || "PASTE_BTC_ADDRESS"

# 1. Validate payout address
valid = np.validate_payout_address(
  address: payout_address,
  currency: pay_currency
)
puts "Address valid? #{valid}"

# 2. Get JWT (payouts require auth)
email = ENV["EMAIL"]
password = ENV["PASSWORD"]
if !email || !password
  puts "Set EMAIL and PASSWORD to create payout"
  exit
end

auth = np.get_auth_token(email, password)
token = auth["token"]

# 3. Create payout
payout = np.create_payout(
  {
    withdrawals: [
      { address: payout_address, currency: pay_currency, amount: 0.0001 }
    ],
    ipn_callback_url: "https://yoursite.com/payout-webhook"
  },
  token
)

puts "Payout created: #{payout["id"]}"
withdrawals = payout["withdrawals"] || []
batch_id = withdrawals[0]&.dig("batch_withdrawal_id") || payout["id"]
puts "Batch withdrawal ID: #{batch_id}"

# 4. Verify (requires verification_code from email)
code = ENV["VERIFICATION_CODE"]
if code
  verified = np.verify_payout(payout["id"], code, token)
  puts "Verified: #{verified}"
else
  puts "Set VERIFICATION_CODE env to verify payout"
end
