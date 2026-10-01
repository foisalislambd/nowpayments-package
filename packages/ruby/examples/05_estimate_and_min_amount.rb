# frozen_string_literal: true

# Get price estimate + minimum amount
# Run: ruby examples/05_estimate_and_min_amount.rb

require_relative "../lib/nowpayments"

api_key = ENV["NOWPAYMENTS_API_KEY"] || "your_api_key"
np = NowPayments::Client.new(api_key: api_key, sandbox: true)

# How much is 100 USD in BTC?
estimate = np.get_estimate_price(
  amount: 100,
  currency_from: "usd",
  currency_to: "btc"
)
puts "100 USD ≈ #{estimate["estimated_amount"]} BTC"

# Minimum payment for USD → BTC
min = np.get_min_amount(
  currency_from: "usd",
  currency_to: "btc",
  fiat_equivalent: "usd"
)
puts "Min amount: #{min["min_amount"]} BTC"
puts "(≈ #{min["fiat_equivalent"]} USD)" if min["fiat_equivalent"]
