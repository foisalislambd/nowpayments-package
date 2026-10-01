# frozen_string_literal: true

# Get available currencies
# Run: ruby examples/06_get_currencies.rb

require_relative "../lib/nowpayments"

api_key = ENV["NOWPAYMENTS_API_KEY"] || "your_api_key"
np = NowPayments::Client.new(api_key: api_key, sandbox: true)

currencies = np.get_currencies
list = currencies["currencies"] || []
puts "Supported: #{list.first(15).join(", ")} ..."
puts "Total: #{list.size}"

# Single currency info
btc_info = np.get_currency("btc")
puts "\nBTC info: #{btc_info}"

# Full currency details (id, name, wallet_regex, network, etc.)
full = np.get_full_currencies
full_list = full["currencies"] || []
btc_full = full_list.find { |c| (c["code"] || "").downcase == "btc" }
puts "\nBTC full: #{btc_full}"
puts "\nMerchant coins: #{np.get_merchant_coins["currencies"]&.first(10)&.join(", ")}..."
