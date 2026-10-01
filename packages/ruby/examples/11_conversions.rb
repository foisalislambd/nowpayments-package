# frozen_string_literal: true

# Create conversion (custody) and check status
# Run: ruby examples/11_conversions.rb
# Env: NOWPAYMENTS_API_KEY, EMAIL, PASSWORD (JWT required)

require_relative "../lib/nowpayments"

api_key = ENV["NOWPAYMENTS_API_KEY"] || "your_api_key"
np = NowPayments::Client.new(api_key: api_key, sandbox: true)

email = ENV["EMAIL"]
password = ENV["PASSWORD"]
if !email || !password
  puts "Set EMAIL and PASSWORD (conversions require JWT)"
  exit
end

auth = np.get_auth_token(email, password)
token = auth["token"]

conv = np.create_conversion(
  {
    amount: 0.001,
    from_currency: "btc",
    to_currency: "usd"
  },
  token
)

puts "Conversion: #{conv}"
if conv && conv["deposit_id"]
  status = np.get_conversion_status(conv["deposit_id"], token)
  puts "Status: #{status}"
end
